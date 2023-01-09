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
export type QueryUser = [] | [
  { 'username' : [] | [string], 'image' : [] | [Uint8Array] }
];
export type Status = { 'InvalidInput' : null } |
  { 'Success' : null } |
  { 'UnAuthorized' : null };
export interface Tree {
  'vertices' : Array<[Uint8Array, FileNode]>,
  'adjacency' : Array<[Uint8Array, Array<Uint8Array>]>,
}
export interface UpdateRespone { 'status' : Status, 'message' : string }
export interface User { 'image' : Uint8Array }
export interface _SERVICE {
  'create_directory' : ActorMethod<[], UpdateRespone>,
  'create_file' : ActorMethod<[CreateFileData], undefined>,
  'get_directories' : ActorMethod<[], FileDirectory>,
  'get_profile' : ActorMethod<[], QueryUser>,
  'register' : ActorMethod<[string], UpdateRespone>,
  'test_ic' : ActorMethod<[], string>,
  'update_profile' : ActorMethod<[User], UpdateRespone>,
}
