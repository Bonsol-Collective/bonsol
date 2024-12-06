{
  description = ''
    Build and develop Bonsol programs without jeoprodizing your existing Solana or Risc0 toolchain.
    Ensure all dependencies align with the requirements set by the Bonsol project.
  '';

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    nix-core = {
      url = "github:Cloud-Scythe-Labs/nix-core";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, nix-core, advisory-db, ... }:
    with flake-utils.lib;
    eachSystem
      (with system; [
        # Currently only known to run on x86-linux but this may change soon
        x86_64-linux
      ])
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          inherit (pkgs) lib;

          rustToolchain = nix-core.toolchains.${system}.mkRustToolchainFromTOML
            ./rust-toolchain.toml
            "sha256-VZZnlyP69+Y3crrLHQyJirqlHrTtGTsyiSnZB8jEvVo=";
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain.fenix-pkgs;
          workspace = rec {
            root = ./.;
            src = craneLib.cleanCargoSource root;
            mkCratePath = crate: root + "/${crate}";
          };

          # Returns true if the dependency requires `risc0-circuit-recursion` as part of its build.
          isRisc0CircuitRecursion = p: lib.hasPrefix
            "git+https://github.com/anagrambuild/risc0?branch=v1.0.1-bonsai-fix#189829d0b84d57e8928a85aa4fac60dd6ce45ea9"
            p.source;
          # Pre-pull the zkr file in order to apply in the postPatch phase for dependencies that require `risc0-circuit-recursion`.
          risc0CircuitRecursionPatch =
            let
              # see https://github.com/risc0/risc0/blob/v1.0.5/risc0/circuit/recursion/build.rs
              sha256Hash = "4e8496469e1efa00efb3630d261abf345e6b2905fb64b4f3a297be88ebdf83d2";
              recursionZkr = pkgs.fetchurl {
                name = "recursion_zkr.zip";
                url = "https://risc0-artifacts.s3.us-west-2.amazonaws.com/zkr/${sha256Hash}.zip";
                hash = "sha256-ToSWRp4e+gDvs2MNJhq/NF5rKQX7ZLTzope+iOvfg9I=";
              };
            in
            ''
              ln -sf ${recursionZkr} ./risc0/circuit/recursion/src/recursion_zkr.zip
            '';
          # Patch git dependencies that require `risc0-circuit-recursion` for bonsol specifically.
          cargoVendorDir = craneLib.vendorCargoDeps (workspace // {
            overrideVendorGitCheckout = ps: drv:
              if lib.any (p: (isRisc0CircuitRecursion p)) ps then
              # Apply the patch for fetching the zkr zip file.
                drv.overrideAttrs
                  {
                    postPatch = risc0CircuitRecursionPatch;
                  }
              else
              # Nothing to change, leave the derivations as is.
                drv;
          });

          # Common arguments can be set here to avoid repeating them later
          commonArgs = {
            inherit (workspace) src;
            inherit cargoVendorDir;
            strictDeps = true;

            nativeBuildInputs = with pkgs; [
              pkg-config
              perl
              autoPatchelfHook
            ] ++ [ flatc ];

            buildInputs = with pkgs; [
              openssl.dev
              libgcc
              libclang.lib
            ];
          };

          # Build *just* the cargo dependencies (of the entire workspace),
          # so we can reuse all of that work (e.g. via cachix) when running in CI
          # It is *highly* recommended to use something like cargo-hakari to avoid
          # cache misses when building individual top-level-crates
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          individualCrateArgs = commonArgs // {
            inherit cargoArtifacts;
            inherit (craneLib.crateNameFromCargoToml { inherit (workspace) src; }) version;
            doCheck = false;
          };

          # Function for including a set of files for a specific crate,
          # avoiding unnecessary files.
          fileSetForCrate = crate: lib.fileset.toSource {
            inherit (workspace) root;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./schemas
              ./schemas-rust
              ./iop
              ./cli
              ./sdk
              ./node
              ./onchain
              ./prover
              ./tester
              (workspace.mkCratePath crate)
            ];
          };

          # Build the top-level crates of the workspace as individual derivations.
          # This allows consumers to only depend on (and build) only what they need.
          # Though it is possible to build the entire workspace as a single derivation,
          # in this case the workspace itself is not a package.
          mkCrateDrv = name: crate:
            let
              manifest = craneLib.crateNameFromCargoToml {
                cargoToml = ((workspace.mkCratePath crate) + "/Cargo.toml");
              };
            in
            craneLib.buildPackage (individualCrateArgs // {
              inherit (manifest) pname;
              cargoExtraArgs = "--locked --bin ${name}";
              src = fileSetForCrate crate;
            });

          bonsol-cli = mkCrateDrv "bonsol" "cli";
          bonsol-node = mkCrateDrv "bonsol-node" "node";

          node_toml = pkgs.callPackage ./nixos/pkgs/bonsol/Node.toml.nix { inherit risc0-groth16-prover; };
          validator = pkgs.callPackage ./nixos/pkgs/bonsol/validator.nix { };
          run-node = pkgs.callPackage ./nixos/pkgs/bonsol/run-node.nix { inherit bonsol-node node_toml; };

          # System dependencies that are pinned to the version that bonsol relies on.
          flatc = with pkgs;
            (flatbuffers.overrideAttrs (old: rec {
              version = "24.3.25";
              src = fetchFromGitHub {
                inherit (old.src) owner repo;
                rev = "v${version}";
                hash = "sha256-uE9CQnhzVgOweYLhWPn2hvzXHyBbFiFVESJ1AEM3BmA=";
              };
            }));
          cargo-risczero = pkgs.callPackage ./nixos/pkgs/risc0/cargo-risczero {
            inherit risc0CircuitRecursionPatch;
          } {
            version = "1.0.1";
            gitHash = "sha256-0Y7+Z2TEm5ZbEkbO8nSOZulGuZAgl9FdyEVNmqV7S8U=";
            cargoHash = "sha256-G3S41Je4HJCvaixjPpNWnHHJgEjTVj83p5xLkXVsASU=";
          };
          r0vm = pkgs.callPackage ./nixos/pkgs/risc0/r0vm {
            inherit risc0CircuitRecursionPatch;
          } {
            version = "1.0.1";
            gitHash = "sha256-0Y7+Z2TEm5ZbEkbO8nSOZulGuZAgl9FdyEVNmqV7S8U=";
            cargoHash = "sha256-3DwrWkjPCE4f/FHjzWyRGAXJPv30B4Ce8fh2oKDhpMM=";
          };
          risc0-groth16-prover = pkgs.callPackage ./nixos/pkgs/risc0/groth16-prover { } {
            imageDigest = "sha256:5a862bac2c5c070ec50ff615572a05d870c1372818cf0f5d8bb9effc101590c8";
            sha256 = "sha256-SV8nUjtq6TheYW+vQltyApOa7/gxnBrWx4Y6fQ71LFg=";
            finalImageTag = "v2024-05-17.1";
          };
          solana-platform-tools = pkgs.callPackage ./nixos/pkgs/solana/platform-tools { } {
            version = "1.41";
            hash = "sha256-m+9QArPvapnOO9lMWYZK2/Yog5cVoY9x1DN7JAusYsk=";
          };
          solana-cli = pkgs.callPackage ./nixos/pkgs/solana { inherit solana-platform-tools; } {
            version = "1.18.22";
            hash = "sha256-MQcnxMhlD0a2cQ8xY//2K+EHgE6rvdUtqufhOw6Ib0Y=";
          };
        in
        {
          checks = {
            # Build the crates as part of `nix flake check` for convenience
            inherit
              bonsol-cli
              bonsol-node;

            # Run clippy (and deny all warnings) on the workspace source
            # TODO: uncomment once all clippy lints are fixed
            # workspace-clippy = craneLib.cargoClippy (commonArgs // {
            #   inherit cargoArtifacts;
            #   cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            # });

            # TODO: Broken because schemas-rust is not generated at this point
            # workspace-doc = craneLib.cargoDoc (commonArgs // {
            #   inherit cargoArtifacts;
            # });

            # Check formatting
            # TODO: Broken because schemas-rust is not generated at this point
            # workspace-fmt = craneLib.cargoFmt {
            #   inherit (workspace) src;
            # };

            workspace-toml-fmt = craneLib.taploFmt {
              src = pkgs.lib.sources.sourceFilesBySuffices workspace.src [ ".toml" ];
            };

            # Audit dependencies
            # TODO: Uncoment once all audits are fixed
            # workspace-audit = craneLib.cargoAudit {
            #   inherit (workspace) src;
            #   inherit advisory-db;
            # };

            # Audit licenses
            # TODO: Many problems still need to be addressed in the deny.toml
            workspace-deny = craneLib.cargoDeny {
              inherit (workspace) src;
            };

            # Run tests with cargo-nextest
            # TODO: Broken because schemas-rust is not generated at this point
            # workspace-nextest = craneLib.cargoNextest (commonArgs // {
            #   inherit cargoArtifacts;
            #   partitions = 1;
            #   partitionType = "count";
            # });
          };

          packages = {
            inherit
              bonsol-cli
              bonsol-node

              validator

              cargo-risczero
              r0vm
              risc0-groth16-prover
              solana-cli
              solana-platform-tools;

            run-node = (run-node.override {
                use-nix = true;
              });

            simple-e2e-script = pkgs.writeShellApplication {
              name = "simple-e2e-test";

              runtimeInputs = with pkgs; [
                docker
                corepack_22
                nodejs_22
                python3
                udev
                rustup
              ] ++ [
                r0vm
                cargo-risczero
                risc0-groth16-prover
                solana-cli
                bonsol-cli
                bonsol-node
                validator
                (run-node.override {
                  use-nix = true;
                })
              ];

              text = ''
                ${bonsol-cli}/bin/bonsol --keypair $HOME/.config/solana/id.json --rpc-url http://localhost:8899 build -z images/simple
                echo "building validator"
                ${validator}/bin/validator.sh > /dev/null 2>&1 &
                validator_pid=$!
                sleep 30
                echo "validator is running: PID $validator_pid"
                echo "building node"
                ${run-node}/bin/run-node.sh > /dev/null 2>&1 &
                node_pid=$!
                sleep 30
                echo "node is running: PID $node_pid"
                ${bonsol-cli}/bin/bonsol --keypair $HOME/.config/solana/id.json --rpc-url http://localhost:8899 deploy url https://bonsol-public-images.s3.amazonaws.com/simple-68f4b0c5f9ce034aa60ceb264a18d6c410a3af68fafd931bcfd9ebe7c1e42960 -m images/simple/manifest.json -y
                sleep 20
                resp=$(${bonsol-cli}/bin/bonsol --keypair $HOME/.config/solana/id.json --rpc-url http://localhost:8899 execute -f testing-examples/example-execution-request.json -x 2000 -m 2000 -w)
                echo "execution response was: $resp"
                kill $validator_pid
                kill $node_pid
                if [[ "$resp" =~ "Success" ]]; then
                  exit 0
                else
                  echo "response was not success"
                  exit 1
                fi
              '';

              checkPhase = "true";
            };
          };

          apps = { };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              # TODO: Remove these once rustup toolchains are linked
              # cargo-hakari
              taplo
              cargo-deny
              cargo-audit
              cargo-nextest

              nil # nix lsp
              nixpkgs-fmt # nix formatter
              # TODO: use `rustup toolchain link` to link fenix toolchain to rustup as the override toolchain
              rustup

              # `setup.sh` dependencies
              docker
              corepack_22
              nodejs_22
              python3
              udev
            ] ++ [
              validator
              run-node

              r0vm
              cargo-risczero
              risc0-groth16-prover
              solana-cli
              flatc
            ];

            # Useful for debugging, sets the path that `cargo-build-sbf` will use to find `platform-tools`
            #
            # SBF_SDK_PATH = "${solana-cli}/bin/sdk/sbf"; # This is the default
          };

          # Run nix fmt to format nix files in file tree
          # using the specified formatter
          formatter = pkgs.nixpkgs-fmt;
        });
}
