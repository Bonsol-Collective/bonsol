use groth16_solana::groth16::Groth16Verifyingkey;

pub const VERIFYINGKEY: Groth16Verifyingkey =  Groth16Verifyingkey {
	nr_pubinputs: 82,

	vk_alpha_g1: [
		45,77,154,167,227,2,217,223,65,116,157,85,7,148,157,5,219,234,51,251,177,108,100,59,34,245,153,162,190,109,242,226,
		20,190,221,80,60,55,206,176,97,216,236,96,32,159,227,69,206,137,131,10,25,35,3,1,240,118,202,255,0,77,25,38,
	],

	vk_beta_g2: [
		9,103,3,47,203,247,118,209,175,201,133,248,136,119,241,130,211,132,128,166,83,242,222,202,169,121,76,188,59,243,6,12,
		14,24,120,71,173,76,121,131,116,208,214,115,43,245,1,132,125,214,139,192,224,113,36,30,2,19,188,127,193,61,183,171,
		48,76,251,209,224,138,112,74,153,245,232,71,217,63,140,60,170,253,222,196,107,122,13,55,157,166,154,77,17,35,70,167,
		23,57,193,177,164,87,168,199,49,49,35,210,77,47,145,146,248,150,183,198,62,234,5,169,213,127,6,84,122,208,206,200,
	],

	vk_gamme_g2: [
		25,142,147,147,146,13,72,58,114,96,191,183,49,251,93,37,241,170,73,51,53,169,231,18,151,228,133,183,174,243,18,194,
		24,0,222,239,18,31,30,118,66,106,0,102,94,92,68,121,103,67,34,212,247,94,218,221,70,222,189,92,217,146,246,237,
		9,6,137,208,88,95,240,117,236,158,153,173,105,12,51,149,188,75,49,51,112,179,142,243,85,172,218,220,209,34,151,91,
		18,200,94,165,219,140,109,235,74,171,113,128,141,203,64,143,227,209,231,105,12,67,211,123,76,230,204,1,102,250,125,170,
	],

	vk_delta_g2: [
		9,134,59,60,28,152,46,34,239,68,3,191,91,104,149,1,158,190,196,163,50,249,163,59,181,53,34,20,26,234,117,18,
		0,181,96,122,223,56,245,185,39,255,138,188,48,74,66,225,160,27,99,176,125,113,175,244,43,112,145,240,251,16,47,173,
		0,43,179,194,241,213,21,175,153,120,250,236,121,68,178,4,181,249,237,177,48,105,56,9,154,10,100,46,237,17,113,156,
		35,138,233,190,94,42,30,30,250,183,140,86,33,70,241,41,90,210,230,242,236,135,194,59,46,22,164,77,223,7,124,132,
	],

	vk_ic: &[
		[
			18,83,241,237,76,37,182,26,157,187,14,13,69,109,177,196,93,51,127,100,180,140,189,58,40,24,180,245,138,108,36,5,
			1,84,188,99,202,102,12,167,148,148,39,136,159,112,132,11,127,90,52,213,235,109,240,44,56,191,183,93,110,207,12,81,
		],
		[
			31,202,1,231,238,175,14,193,180,159,179,216,7,236,98,30,232,137,129,139,197,18,94,236,59,96,58,15,137,106,231,96,
			34,99,244,212,10,115,156,190,176,190,205,166,145,146,67,144,104,188,225,47,51,186,11,186,189,85,127,131,8,194,225,202,
		],
		[
			12,221,100,171,44,118,250,56,255,157,252,158,244,73,56,21,109,170,186,22,32,58,178,38,124,102,237,147,177,67,215,99,
			36,254,211,14,131,145,233,53,166,2,19,3,127,35,225,83,188,250,36,177,149,69,120,253,224,112,41,61,242,184,80,110,
		],
		[
			11,66,136,70,162,208,228,172,152,134,212,43,249,76,170,165,227,176,224,95,152,74,178,241,22,152,234,96,84,243,213,118,
			13,239,36,206,41,122,27,219,198,70,209,73,142,72,117,207,178,45,161,30,168,76,2,63,240,66,39,167,250,187,200,36,
		],
		[
			10,220,115,147,152,241,37,114,248,158,101,169,147,45,214,5,23,7,76,76,216,99,116,246,234,150,200,241,37,197,124,68,
			24,178,101,55,140,91,180,163,21,102,214,34,228,1,8,14,150,144,31,159,214,223,177,228,137,177,87,109,0,141,35,30,
		],
		[
			3,134,229,199,11,62,166,73,220,51,91,230,239,253,89,204,54,209,88,102,183,206,135,187,98,229,9,179,243,3,113,6,
			8,217,31,232,208,99,16,124,177,157,114,147,174,81,249,85,226,166,252,240,26,63,73,32,206,92,30,226,127,14,7,253,
		],
		[
			10,184,94,54,111,5,33,39,208,195,11,130,141,241,122,162,128,183,79,1,65,28,115,124,126,214,57,115,211,144,103,196,
			39,9,27,66,170,160,212,25,51,78,78,229,119,15,0,135,110,164,142,14,35,221,246,186,87,193,83,69,193,110,106,71,
		],
		[
			34,45,40,6,137,63,161,30,39,4,117,77,43,238,249,236,32,156,19,95,183,123,240,60,47,237,32,204,161,65,135,24,
			40,46,245,84,232,176,37,104,6,138,202,166,221,191,202,118,114,220,173,73,162,148,10,112,140,5,19,135,117,121,103,34,
		],
		[
			36,47,200,253,173,168,98,153,125,102,99,5,0,209,24,216,16,57,93,243,100,148,74,107,190,130,147,223,80,19,102,42,
			37,129,101,198,177,202,68,195,231,188,14,57,50,236,82,179,175,57,160,27,183,138,44,199,24,9,252,25,7,178,14,131,
		],
		[
			8,3,143,223,228,63,106,115,47,84,222,69,61,11,202,61,113,145,240,74,107,235,92,204,194,86,94,233,124,111,57,233,
			4,94,240,51,76,214,7,1,39,172,184,193,147,58,24,165,201,107,170,60,112,115,211,75,26,1,253,222,8,86,76,61,
		],
		[
			38,155,145,84,204,196,226,85,117,15,209,44,33,168,45,89,56,197,55,106,160,140,125,236,51,39,196,55,89,122,155,244,
			37,47,200,16,52,19,250,133,142,95,90,217,156,214,111,108,191,21,140,198,244,41,187,175,16,26,31,234,116,20,179,33,
		],
		[
			20,231,135,184,139,97,30,231,196,142,161,81,233,159,78,50,197,2,5,142,164,114,118,70,45,5,214,186,156,191,152,139,
			25,71,77,84,235,120,226,142,9,61,26,169,90,206,109,195,23,247,172,135,128,131,220,37,182,151,58,90,6,104,255,194,
		],
		[
			1,146,225,45,14,140,157,99,106,172,33,70,128,150,245,205,86,217,80,167,216,194,96,146,107,33,27,116,127,149,3,167,
			40,125,185,117,61,187,92,100,234,82,100,57,77,218,188,234,82,160,4,199,87,219,47,165,167,195,44,79,160,216,1,63,
		],
		[
			32,76,22,124,19,40,2,37,187,189,189,219,12,233,149,25,40,106,36,182,162,227,145,78,171,7,94,68,101,199,247,3,
			5,245,93,242,71,6,235,61,13,116,122,190,2,246,205,209,70,181,36,135,230,211,118,95,42,73,87,33,112,22,12,103,
		],
		[
			18,194,244,33,44,225,237,53,5,13,104,251,173,160,227,89,243,157,190,220,194,226,102,50,248,150,118,44,82,140,205,251,
			14,34,121,206,171,164,142,232,115,168,114,230,132,229,8,25,201,236,124,198,155,85,62,151,21,197,174,16,206,87,249,197,
		],
		[
			13,182,90,60,41,139,225,27,33,228,200,51,129,156,122,176,78,137,119,171,45,128,75,227,156,176,55,91,255,226,216,255,
			25,76,125,48,193,104,120,242,166,106,91,116,42,246,214,171,194,240,196,127,255,37,10,127,201,28,231,153,252,195,41,17,
		],
		[
			5,93,220,247,142,247,136,13,114,197,22,94,141,67,53,220,200,45,161,207,216,84,63,113,184,49,137,103,143,209,7,63,
			13,217,222,107,93,167,245,128,254,241,35,93,172,62,158,240,39,29,20,92,210,241,120,193,70,20,142,201,127,99,212,195,
		],
		[
			44,17,198,106,234,100,248,160,253,146,196,44,9,30,23,248,207,153,202,167,21,117,186,31,65,244,75,97,65,165,242,10,
			44,150,243,228,198,88,60,124,21,178,152,165,1,196,244,247,255,69,158,200,205,38,199,236,74,229,76,14,227,80,235,96,
		],
		[
			28,13,221,198,146,25,239,155,52,48,231,174,136,140,218,194,44,23,58,248,99,178,93,82,191,50,80,144,39,204,184,105,
			11,171,36,173,202,81,234,63,237,138,92,150,254,204,64,176,32,189,180,140,44,231,108,253,164,241,71,14,214,146,184,151,
		],
		[
			21,41,169,164,131,127,129,238,107,193,135,69,30,133,244,155,172,247,48,185,225,3,102,140,90,168,14,241,62,26,115,239,
			34,115,216,19,149,227,61,95,163,20,97,14,123,6,220,123,219,9,93,36,16,102,191,127,234,70,211,79,106,8,143,70,
		],
		[
			6,75,142,9,176,135,200,124,220,242,54,150,107,53,119,241,50,213,170,96,181,195,241,59,32,29,181,144,185,83,217,47,
			22,136,167,73,39,251,99,119,173,62,199,197,85,105,19,217,156,172,80,55,214,9,254,238,152,92,193,166,240,171,32,221,
		],
		[
			6,245,180,168,24,232,91,89,101,60,145,209,46,108,202,98,135,223,118,205,221,3,88,58,31,93,80,36,138,200,168,213,
			28,12,81,222,76,48,172,48,124,61,226,100,126,27,36,93,62,108,247,195,149,143,188,165,183,135,88,155,138,220,238,59,
		],
		[
			33,200,127,248,229,122,164,241,238,134,233,64,161,1,82,85,18,36,42,108,16,63,255,116,191,80,211,45,235,251,12,51,
			10,165,77,29,248,74,229,97,238,250,189,13,156,117,43,188,171,237,241,150,180,22,117,207,54,148,157,223,198,105,105,34,
		],
		[
			5,115,78,193,135,121,24,26,147,26,165,199,166,135,95,119,206,121,207,190,24,109,107,141,86,148,71,198,133,191,37,60,
			44,123,165,198,111,130,63,143,115,161,2,174,5,89,174,127,8,198,36,103,165,154,109,23,223,86,88,181,232,215,34,32,
		],
		[
			33,0,91,249,224,38,97,241,14,135,8,57,241,3,206,90,89,234,11,212,225,107,128,60,235,196,109,20,14,37,41,203,
			16,148,143,169,254,172,249,149,240,27,166,97,136,41,107,1,181,179,174,209,29,30,199,62,53,179,29,166,86,51,90,91,
		],
		[
			12,126,217,103,93,106,87,212,222,195,42,52,11,176,187,226,39,75,211,116,64,100,95,221,33,120,124,7,62,251,36,97,
			44,143,241,165,182,170,24,222,123,249,52,162,86,253,228,217,131,52,117,254,39,43,37,53,53,116,206,141,202,14,199,198,
		],
		[
			16,151,68,136,10,238,123,170,121,42,206,4,179,194,129,107,154,27,230,214,83,251,157,244,144,213,159,112,103,55,161,157,
			19,142,236,168,253,97,114,224,26,241,119,29,106,25,120,43,108,152,99,34,218,108,127,112,41,56,44,104,22,62,159,191,
		],
		[
			17,31,138,211,217,166,233,92,202,243,119,231,206,167,242,16,192,243,106,40,215,248,61,12,194,11,228,9,215,156,148,132,
			4,147,219,33,86,96,179,74,190,174,53,155,2,220,165,39,119,227,101,209,205,82,227,245,58,154,144,115,166,105,184,175,
		],
		[
			44,111,164,217,93,33,130,228,1,84,12,220,80,172,3,103,30,35,63,77,231,112,63,170,55,165,71,26,224,135,124,198,
			37,228,124,15,205,42,22,24,154,223,12,54,193,82,176,7,242,164,121,211,60,228,117,79,221,122,113,52,153,245,136,199,
		],
		[
			3,147,95,149,118,28,205,46,54,49,21,92,43,226,71,129,33,39,239,116,238,81,100,195,136,208,192,151,169,204,40,196,
			0,2,155,81,252,163,252,69,185,148,165,45,200,46,36,110,72,39,193,210,194,246,213,65,221,197,113,55,242,112,137,118,
		],
		[
			24,225,238,79,167,110,222,195,196,139,46,161,107,2,229,87,67,173,219,14,223,69,120,186,186,92,28,154,176,126,174,197,
			28,188,50,190,92,4,230,200,37,184,127,184,166,177,162,41,100,95,220,113,147,214,103,93,103,95,127,194,106,21,77,17,
		],
		[
			28,53,207,243,99,34,88,143,74,84,63,70,77,190,80,127,100,203,14,213,136,183,62,104,173,21,159,20,23,251,7,124,
			7,35,163,192,169,6,136,24,18,59,143,227,165,213,5,104,97,8,215,133,25,165,249,132,157,107,242,121,215,43,160,163,
		],
		[
			38,233,222,153,217,24,183,232,62,228,10,164,251,157,109,22,3,85,67,88,150,175,100,13,210,224,155,47,53,150,245,89,
			38,225,128,82,188,88,8,30,245,162,105,188,206,53,223,240,182,9,56,11,139,72,126,35,101,152,93,253,203,65,130,179,
		],
		[
			7,4,79,232,200,110,7,214,209,100,219,63,54,93,231,41,49,93,102,14,117,195,55,52,25,55,147,126,131,103,3,243,
			13,217,31,217,75,255,13,172,4,86,205,99,228,107,199,192,91,29,212,228,237,189,31,122,139,219,22,62,222,234,16,153,
		],
		[
			32,173,131,27,240,38,170,211,141,40,25,142,141,223,195,11,152,152,45,230,171,68,16,156,127,127,10,200,46,106,51,163,
			17,153,116,39,255,29,20,73,98,246,218,184,164,17,190,51,29,211,237,254,235,173,0,104,198,76,234,228,31,34,239,67,
		],
		[
			22,234,205,85,48,28,184,152,182,104,164,215,2,129,135,39,4,105,8,100,179,156,214,146,187,63,96,208,15,145,80,207,
			36,98,51,91,156,242,48,221,199,193,73,47,125,108,38,9,203,24,54,105,159,116,209,220,99,186,147,57,101,95,34,92,
		],
		[
			31,88,20,59,66,10,210,152,160,49,129,115,33,31,132,243,219,238,234,16,27,230,145,232,208,32,90,206,52,151,183,5,
			12,1,214,226,56,199,82,32,199,238,112,183,11,154,166,245,37,98,58,174,232,163,5,155,50,131,78,86,226,168,225,83,
		],
		[
			0,165,246,105,179,79,185,73,221,45,27,230,248,175,175,4,78,169,246,93,117,193,119,230,57,54,13,115,206,20,171,181,
			32,227,73,41,238,127,22,59,23,51,154,29,227,151,129,7,22,59,114,108,232,8,40,113,3,209,62,237,14,214,178,216,
		],
		[
			41,143,8,112,213,158,138,155,139,157,48,200,88,107,192,165,27,164,228,148,46,250,43,17,252,112,36,217,167,34,142,175,
			44,220,58,190,55,180,217,159,4,17,5,89,9,199,208,115,195,31,149,52,224,166,172,191,154,226,218,231,71,40,193,225,
		],
		[
			44,31,254,235,55,188,248,101,151,172,223,218,179,77,158,22,6,187,178,186,88,171,234,45,76,201,121,212,92,169,177,220,
			27,112,182,97,193,130,80,111,225,175,47,147,117,10,231,67,150,177,227,175,217,72,167,129,181,189,226,182,206,33,158,240,
		],
		[
			30,75,141,89,43,146,230,144,144,210,30,97,123,234,105,33,45,54,181,24,76,99,38,219,5,251,85,193,202,82,134,232,
			1,7,244,216,220,104,74,249,99,5,244,41,68,124,223,28,51,151,238,46,254,6,226,241,34,190,81,128,115,244,141,173,
		],
		[
			3,117,154,12,184,122,116,133,31,210,58,38,182,6,48,77,63,255,232,128,209,234,234,171,105,220,194,243,180,102,34,6,
			44,192,102,237,65,26,214,30,87,203,193,11,142,82,201,17,84,177,129,223,136,12,76,214,5,73,26,147,43,205,217,153,
		],
		[
			35,85,191,249,251,175,5,202,96,219,9,172,154,240,31,52,76,165,127,25,195,224,217,210,52,177,244,138,78,12,94,62,
			38,241,17,115,245,236,103,175,239,239,10,50,88,44,49,191,157,155,10,143,166,104,80,199,61,160,197,97,130,53,235,89,
		],
		[
			46,197,76,132,166,60,165,134,105,195,228,244,202,173,155,142,188,168,113,76,166,67,22,71,91,204,62,129,17,49,124,120,
			23,205,132,137,120,200,171,2,157,106,84,221,238,101,94,211,186,17,26,255,35,156,252,240,53,99,151,184,74,162,38,105,
		],
		[
			46,81,18,232,242,98,58,161,177,70,235,35,111,247,150,106,171,41,179,58,37,13,198,210,106,37,106,73,98,224,204,121,
			7,198,249,243,16,246,192,104,94,41,194,63,12,179,44,166,95,192,56,164,92,216,249,245,66,145,7,190,69,245,94,93,
		],
		[
			36,155,120,216,46,245,46,239,236,0,56,7,177,169,71,63,179,206,189,130,176,197,123,153,162,168,158,141,21,112,85,143,
			7,185,58,13,86,70,241,191,54,227,29,247,215,89,208,211,182,162,168,148,178,125,35,61,238,3,229,150,121,236,28,14,
		],
		[
			27,194,139,60,48,207,253,66,116,74,166,3,133,181,14,233,225,179,6,69,34,157,112,25,186,84,75,25,254,22,172,136,
			0,253,135,194,209,199,58,102,33,136,243,52,230,65,188,205,28,40,108,242,237,34,182,67,175,166,68,161,48,234,81,246,
		],
		[
			18,122,157,2,102,80,40,220,93,171,198,254,128,15,48,32,28,143,67,215,202,9,245,212,186,27,12,21,63,82,80,154,
			10,123,63,150,157,135,202,0,215,248,231,96,46,205,65,133,185,6,235,37,169,101,39,245,164,245,42,90,38,182,146,213,
		],
		[
			12,132,128,241,40,156,199,191,102,162,67,137,97,7,89,36,51,52,113,164,245,134,78,250,241,150,229,152,62,158,10,89,
			1,101,74,138,45,31,235,146,68,217,74,123,13,59,73,160,91,230,51,106,30,67,91,123,2,34,175,157,92,18,38,24,
		],
		[
			2,30,130,216,200,2,185,153,35,53,96,118,207,221,12,111,38,173,244,157,117,95,91,71,36,228,180,96,41,199,130,50,
			47,192,78,239,38,149,30,9,203,49,108,246,81,87,255,36,141,168,144,17,173,139,195,151,110,98,190,192,166,62,172,30,
		],
		[
			20,7,71,133,5,79,234,65,152,255,10,75,182,117,58,75,250,134,204,51,233,211,8,185,208,122,12,87,135,152,182,9,
			17,39,123,60,208,44,39,174,178,168,182,112,154,172,243,143,198,61,181,146,115,90,60,197,28,44,147,40,40,60,67,109,
		],
		[
			39,203,190,251,208,89,150,52,159,99,82,224,204,227,96,233,209,186,195,224,72,120,197,71,237,133,185,178,240,56,130,107,
			20,108,89,68,79,125,231,74,139,201,200,120,226,170,86,137,70,17,224,171,90,155,28,26,82,76,171,252,27,55,165,144,
		],
		[
			30,173,64,158,181,57,69,154,203,196,63,1,40,237,29,74,228,201,199,141,100,129,145,76,34,97,148,123,63,118,6,56,
			7,187,99,99,197,202,75,226,218,121,227,71,237,126,0,191,202,66,230,9,98,87,177,62,190,174,18,76,236,175,213,137,
		],
		[
			40,200,208,216,50,52,188,217,231,145,70,97,28,137,240,63,64,252,13,253,170,183,77,60,180,26,251,167,151,120,211,36,
			37,193,110,238,21,47,183,51,29,243,160,85,187,198,80,88,35,149,19,236,130,137,25,180,254,252,115,154,164,179,89,254,
		],
		[
			28,105,112,186,159,129,57,67,178,3,156,221,65,53,120,16,35,78,177,38,148,16,40,43,165,54,147,100,43,198,21,79,
			41,50,134,166,12,140,205,64,214,183,144,207,97,133,147,207,122,18,171,212,95,120,42,5,97,36,206,19,83,101,53,45,
		],
		[
			38,121,199,230,244,41,172,77,239,114,0,81,133,174,126,85,152,71,206,138,204,162,79,71,110,19,235,185,19,253,176,250,
			19,209,152,12,244,92,217,142,95,167,138,209,87,59,11,129,17,169,110,44,160,109,116,43,99,193,255,92,215,160,167,186,
		],
		[
			39,247,12,163,237,19,52,17,60,242,158,161,68,127,196,38,6,182,88,162,124,221,65,56,159,138,240,212,196,128,55,103,
			8,128,242,125,22,98,43,220,39,132,174,96,20,34,62,225,195,17,157,22,86,167,237,9,251,62,92,244,20,201,71,108,
		],
		[
			0,43,223,204,231,116,59,115,165,159,114,192,110,147,65,128,9,172,184,113,25,83,255,90,192,235,46,251,198,116,168,80,
			13,200,84,142,92,111,4,149,200,65,77,29,72,84,73,36,204,165,169,71,110,53,59,194,123,183,174,116,191,190,138,179,
		],
		[
			43,113,180,130,212,19,102,171,63,34,86,118,145,123,125,114,246,249,85,168,133,207,108,99,106,206,222,187,4,163,231,105,
			24,53,138,185,228,179,249,204,217,153,27,139,28,203,68,80,138,103,132,31,196,152,147,190,213,9,45,44,113,165,72,217,
		],
		[
			9,10,197,93,171,240,146,5,112,69,198,147,148,180,6,8,10,65,80,234,50,2,198,170,97,135,72,96,67,56,90,145,
			36,107,57,60,163,215,201,142,237,42,234,209,231,127,74,87,35,132,17,221,180,220,241,129,191,101,249,214,187,70,104,78,
		],
		[
			7,224,63,76,176,189,174,199,221,162,144,40,130,195,193,154,181,163,188,124,143,56,157,8,58,30,110,47,40,102,94,74,
			1,64,117,91,196,26,54,59,138,174,47,138,223,8,61,174,110,28,41,128,151,142,247,34,218,79,214,45,120,143,160,95,
		],
		[
			17,103,63,148,8,29,172,54,21,118,159,4,10,63,227,156,163,244,46,129,107,223,122,114,59,211,240,126,89,30,186,190,
			7,125,194,69,239,175,107,181,58,50,40,193,145,53,130,149,144,246,33,17,73,89,200,26,35,12,181,48,113,161,71,9,
		],
		[
			15,220,68,130,42,95,101,242,65,103,203,236,8,77,30,14,173,212,210,209,248,203,226,236,220,58,189,77,177,76,214,22,
			25,82,129,194,215,58,117,165,164,207,255,85,100,16,201,76,251,157,136,44,137,64,210,215,57,13,218,46,204,206,75,205,
		],
		[
			17,192,214,247,111,91,82,51,174,155,132,52,113,42,159,241,14,244,62,29,150,152,194,4,131,115,181,176,129,65,207,23,
			47,255,146,212,194,177,95,42,181,231,186,12,125,122,51,112,21,89,162,178,58,46,146,174,181,112,191,129,202,150,51,112,
		],
		[
			22,170,124,222,189,190,81,139,99,146,59,226,200,13,157,69,249,162,19,99,53,238,11,16,82,67,226,42,145,38,222,100,
			5,32,44,155,140,192,58,85,68,104,57,187,74,124,142,3,56,127,140,0,133,177,208,166,97,32,163,114,179,187,173,27,
		],
		[
			18,136,126,247,51,219,100,245,129,208,255,19,205,81,252,210,23,75,163,237,221,139,143,104,103,126,160,118,177,185,91,87,
			44,166,84,170,97,156,75,4,78,90,167,12,43,100,111,73,70,89,131,42,8,57,71,100,25,14,250,74,120,212,210,22,
		],
		[
			7,137,233,176,209,176,3,86,20,201,3,101,2,192,61,233,232,164,131,153,118,48,10,87,56,36,199,21,28,199,23,205,
			14,113,178,248,77,250,211,105,99,111,160,154,38,211,6,87,192,37,226,116,148,180,94,249,19,3,22,101,21,44,157,248,
		],
		[
			43,255,53,253,202,57,77,63,33,27,99,159,45,93,160,242,54,209,129,233,30,98,4,168,174,211,169,40,70,64,120,6,
			15,184,204,246,145,181,165,165,66,218,178,243,71,173,180,46,175,244,21,60,71,17,145,75,242,173,98,158,204,84,29,214,
		],
		[
			47,7,47,49,245,142,31,8,66,247,160,60,109,75,231,12,158,92,183,45,41,156,178,28,178,159,193,92,3,162,147,227,
			5,241,171,52,29,206,22,30,156,227,7,99,5,193,164,226,8,221,216,177,174,165,219,6,248,64,162,36,56,52,17,120,
		],
		[
			13,3,175,55,141,212,152,158,71,181,113,135,134,42,27,231,37,248,56,162,4,249,60,9,163,1,56,157,236,235,23,226,
			13,252,121,8,239,98,200,224,17,137,45,82,123,225,225,35,246,117,228,94,82,200,201,32,56,195,22,192,180,66,33,58,
		],
		[
			29,6,178,24,96,225,214,212,82,232,96,50,35,55,42,232,142,112,233,103,42,62,101,74,39,221,23,109,86,162,17,163,
			40,199,30,213,17,60,174,217,110,240,153,206,181,196,181,194,142,53,116,35,244,33,91,185,207,214,67,231,217,148,137,149,
		],
		[
			24,236,233,224,94,236,150,196,232,142,168,159,24,64,87,19,14,148,241,174,154,74,2,115,23,82,146,53,31,58,8,4,
			30,32,47,127,240,54,28,134,232,193,123,90,59,125,130,15,245,25,93,92,226,253,254,152,26,106,139,213,79,101,77,28,
		],
		[
			46,193,235,7,136,203,89,107,64,176,248,125,43,236,94,67,19,241,132,244,161,155,241,21,159,214,235,144,200,224,243,3,
			47,255,47,137,207,32,237,120,100,8,241,9,122,191,135,56,7,10,151,119,188,131,228,186,213,64,75,103,94,79,192,246,
		],
		[
			15,123,92,219,247,7,80,206,226,46,29,184,57,124,8,70,106,181,85,55,99,53,248,68,32,138,173,88,89,100,7,185,
			13,105,97,168,10,135,75,41,255,69,164,36,58,244,121,55,53,31,14,246,0,117,189,106,130,185,182,158,162,81,238,75,
		],
		[
			26,174,138,98,142,198,149,143,29,55,166,244,155,97,152,120,232,88,192,202,106,23,7,121,67,246,162,58,176,118,21,113,
			31,210,109,227,170,147,92,132,7,44,121,50,223,192,248,114,165,63,216,121,159,103,223,94,129,100,131,107,123,208,247,165,
		],
		[
			9,110,185,212,102,164,109,121,210,155,26,67,122,12,135,43,110,185,3,26,255,86,237,40,23,119,152,166,40,12,18,4,
			6,216,180,7,230,55,34,216,60,7,117,250,35,195,197,219,132,247,192,62,184,101,202,177,96,24,186,234,202,75,32,149,
		],
		[
			46,167,18,105,208,100,172,111,254,127,37,18,241,141,8,143,67,233,74,156,58,147,60,75,205,200,101,235,197,50,236,33,
			11,197,112,27,200,103,117,22,181,141,212,121,225,22,21,158,38,139,170,2,153,40,233,75,168,203,13,180,183,93,254,180,
		],
		[
			41,168,216,202,107,246,157,232,179,4,220,79,83,82,191,93,162,14,151,150,229,74,122,162,94,226,164,161,58,166,95,52,
			10,85,178,177,43,182,203,191,196,212,8,233,168,76,60,5,71,219,82,58,178,152,36,194,236,136,68,82,78,15,161,147,
		],
		[
			26,190,122,248,41,32,78,34,12,81,38,155,166,152,252,128,200,104,53,207,0,160,33,151,26,237,76,229,246,105,84,42,
			45,248,154,211,98,214,52,53,238,1,5,4,41,228,143,216,193,55,13,217,223,68,2,20,102,202,212,145,241,207,210,68,
		],
		[
			16,174,17,112,115,220,5,25,67,84,40,160,115,218,7,249,79,179,180,61,39,6,116,85,141,37,175,77,137,82,37,52,
			25,253,170,246,239,162,211,137,251,181,112,83,58,19,233,187,108,15,223,151,35,32,72,26,64,236,252,229,121,62,195,90,
		],
		[
			40,2,244,121,175,40,110,45,156,234,16,204,20,113,193,89,178,147,92,187,173,169,61,158,37,123,201,44,8,96,72,164,
			4,26,181,71,181,161,127,51,202,195,176,142,17,220,21,210,186,105,119,212,66,60,33,187,195,103,35,113,248,138,204,67,
		],
		[
			21,70,122,186,224,209,56,152,76,14,43,49,24,21,99,198,176,169,75,249,161,99,85,2,48,1,2,95,72,27,82,63,
			42,110,247,40,71,206,178,141,65,138,67,156,121,156,97,140,243,211,7,192,137,159,46,109,192,58,117,116,254,103,17,208,
		],
	]
};