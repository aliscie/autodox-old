import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ElementTree { 'id' : string, 'elements' : Tree }
export interface FileDirectory {
  'id' : string,
  'files' : Tree,
  'name' : string,
}
export type FileMode = { 'Private' : null } |
  { 'Public' : null } |
  { 'Restricted' : null };
export interface FileNode {
  'id' : string,
  'name' : string,
  'file_mode' : FileMode,
  'element_tree' : [] | [string],
}
export interface Tree {
  'root' : [] | [string],
  'vertices' : Array<[string, FileNode]>,
  'adjacency' : Array<[string, Array<string>]>,
}
export interface UserQuery {
  'username' : [] | [string],
  'birthdate' : [] | [string],
  'emails' : [] | [Array<string>],
  'email' : [] | [string],
  'address' : string,
  'first_name' : [] | [string],
  'last_name' : [] | [string],
  'image' : [] | [Uint8Array],
}
export interface _SERVICE {
  'change_directory' : ActorMethod<[string], string>,
  'create_directory' : ActorMethod<[], string>,
  'create_element_tree' : ActorMethod<[string], [] | [string]>,
  'create_file' : ActorMethod<[string], string>,
  'delete_file' : ActorMethod<[string], string>,
  'get_directories' : ActorMethod<[], [] | [FileDirectory]>,
  'get_directory' : ActorMethod<[string], [] | [FileDirectory]>,
  'get_element_trees' : ActorMethod<[], [] | [[string, ElementTree]]>,
  'get_file' : ActorMethod<[string], [] | [[FileNode, ElementTree]]>,
  'get_profile' : ActorMethod<[], [] | [UserQuery]>,
  'get_users' : ActorMethod<[], Array<UserQuery>>,
  'group_update' : ActorMethod<[string], [] | [string]>,
  'register' : ActorMethod<[string], string>,
  'rename_file' : ActorMethod<[string], string>,
  'update_file' : ActorMethod<[string], string>,
  'update_profile' : ActorMethod<[string], string>,
}
