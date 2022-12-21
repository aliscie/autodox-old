import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CreateDirectoryData {
  'id' : Uint8Array,
  'files' : Tree,
  'name' : string,
}
export interface CreateFileData {
  'id' : Uint8Array,
  'name' : string,
  'children' : [] | [Array<string>],
  'parent_id' : Uint8Array,
  'directory_id' : Uint8Array,
}
export type ElementId = { 'None' : null } |
  { 'Some' : Uint8Array };
export type FileDirectory = [] | [
  { 'id' : Uint8Array } |
    { 'files' : Tree } |
    { 'name' : string }
];
export type FileDirectoryUpdate = { 'files' : Tree } |
  { 'name' : string };
export type FileMode = { 'Private' : null } |
  { 'Public' : null } |
  { 'Restricted' : null };
export interface FileNode { 'name' : string, 'element_tree' : ElementId }
export type Status = { 'InvalidInput' : null } |
  { 'Success' : null } |
  { 'UnAuthorized' : null };
export interface Tree {
  'vertices' : Array<[Uint8Array, FileNode]>,
  'adjacency' : Array<[Uint8Array, Array<Uint8Array>]>,
}
export interface UpdateRespone { 'status' : Status, 'message' : string }
export interface _SERVICE {
  'create_directory' : ActorMethod<[], UpdateRespone>,
  'create_file' : ActorMethod<[CreateFileData], undefined>,
  'get_directories' : ActorMethod<[], FileDirectory>,
  'register' : ActorMethod<[string], string>,
  'test_ic' : ActorMethod<[], string>,
}
