import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CreateDirectoryData {
  'id' : string,
  'files' : Tree,
  'name' : string,
}
export interface CreateFileData {
  'id' : string,
  'name' : string,
  'children' : [] | [Array<string>],
  'parent_id' : string,
  'directory_id' : string,
}
export interface FileDirectory {
  'id' : string,
  'files' : Tree,
  'name' : string,
}
export type FileDirectoryUpdate = { 'files' : Tree } |
  { 'name' : string };
export type FileMode = { 'Private' : null } |
  { 'Public' : null } |
  { 'Restricted' : null };
export interface FileNode {
  'id' : string,
  'name' : string,
  'element_tree' : [] | [string],
}
export type QueryUser = [] | [
  { 'username' : [] | [string], 'image' : [] | [Uint8Array] }
];
export type Status = { 'InvalidInput' : null } |
  { 'Success' : null } |
  { 'UnAuthorized' : null };
export interface Tree {
  'root' : [] | [string],
  'vertices' : Array<[string, FileNode]>,
  'adjacency' : Array<[string, Array<Uint8Array>]>,
}
export interface UpdateRespone { 'status' : Status, 'message' : string }
export interface User { 'username' : string, 'image' : Uint8Array }
export interface _SERVICE {
  'create_directory' : ActorMethod<[], UpdateRespone>,
  'create_file' : ActorMethod<[CreateFileData], undefined>,
  'get_directories' : ActorMethod<[], Array<FileDirectory>>,
  'get_profile' : ActorMethod<[], QueryUser>,
  'register' : ActorMethod<[string], UpdateRespone>,
  'test_ic' : ActorMethod<[], string>,
  'update_profile' : ActorMethod<[User], UpdateRespone>,
}
