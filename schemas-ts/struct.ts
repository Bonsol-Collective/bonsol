// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

export class Struct {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):Struct {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsStruct(bb:flatbuffers.ByteBuffer, obj?:Struct):Struct {
  return (obj || new Struct()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsStruct(bb:flatbuffers.ByteBuffer, obj?:Struct):Struct {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new Struct()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static startStruct(builder:flatbuffers.Builder) {
  builder.startObject(0);
}

static endStruct(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createStruct(builder:flatbuffers.Builder):flatbuffers.Offset {
  Struct.startStruct(builder);
  return Struct.endStruct(builder);
}
}
