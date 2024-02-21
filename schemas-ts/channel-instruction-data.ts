// automatically generated by the FlatBuffers compiler, do not modify

import { ExecutionRequestV1 } from './execution-request-v1.js';
import { StatusV1 } from './status-v1.js';


export enum ChannelInstructionData {
  NONE = 0,
  ExecuteV1 = 1,
  StatusV1 = 2
}

export function unionToChannelInstructionData(
  type: ChannelInstructionData,
  accessor: (obj:ExecutionRequestV1|StatusV1) => ExecutionRequestV1|StatusV1|null
): ExecutionRequestV1|StatusV1|null {
  switch(ChannelInstructionData[type]) {
    case 'NONE': return null; 
    case 'ExecuteV1': return accessor(new ExecutionRequestV1())! as ExecutionRequestV1;
    case 'StatusV1': return accessor(new StatusV1())! as StatusV1;
    default: return null;
  }
}

export function unionListToChannelInstructionData(
  type: ChannelInstructionData, 
  accessor: (index: number, obj:ExecutionRequestV1|StatusV1) => ExecutionRequestV1|StatusV1|null, 
  index: number
): ExecutionRequestV1|StatusV1|null {
  switch(ChannelInstructionData[type]) {
    case 'NONE': return null; 
    case 'ExecuteV1': return accessor(index, new ExecutionRequestV1())! as ExecutionRequestV1;
    case 'StatusV1': return accessor(index, new StatusV1())! as StatusV1;
    default: return null;
  }
}
