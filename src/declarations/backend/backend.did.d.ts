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
  { 'id' : Uint8Array, 'files' : Tree, 'name' : string }
];
export type FileMode = { 'Private' : null } |
  { 'Public' : null } |
  { 'Restricted' : null };
export interface FileNode { 'name' : string, 'element_tree' : ElementId }
export interface ResponseQuery { 'status' : Status, 'message' : string }
export type Status = { 'InvalidInput' : null } |
  { 'Success' : null } |
  { 'UnAuthorized' : null };
export type TestQuery = [] | [
  { 'username' : [] | [string], 'image' : [] | [string] }
];
export interface Tree {
  'vertices' : Array<[Uint8Array, FileNode]>,
  'adjacency' : Array<[Uint8Array, Array<Uint8Array>]>,
}
export type UserQuery = [] | [
  { 'username' : [] | [string], 'image' : [] | [Uint8Array] }
];
export interface UserUpdate { 'username' : string, 'image' : Uint8Array }
export interface _SERVICE {
  'create_directory' : ActorMethod<[], ResponseQuery>,
  'create_file' : ActorMethod<[CreateFileData], undefined>,
  'get_current_user' : ActorMethod<[], UserQuery>,
  'get_directories' : ActorMethod<[], FileDirectory>,
  'get_profile' : ActorMethod<[], UserQuery>,
  'get_test' : ActorMethod<[], TestQuery>,
  'register' : ActorMethod<[string], ResponseQuery>,
  'test_ic' : ActorMethod<[], string>,
  'update_profile' : ActorMethod<[UserUpdate], ResponseQuery>,
}
