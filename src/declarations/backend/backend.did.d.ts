import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface EditorElement {
  'id' : string,
  'tag' : [] | [string],
  'content' : string,
  'attrs' : Array<[string, string]>,
}
export interface ElementTree { 'id' : string, 'elements' : Tree_1 }
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
export type Result = { 'Ok' : string } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : Array<[string, ElementTree]> } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : [FileNode, ElementTree] } |
  { 'Err' : string };
export interface Tree {
  'root' : [] | [string],
  'vertices' : Array<[string, FileNode]>,
  'adjacency' : Array<[string, Array<string>]>,
}
export interface Tree_1 {
  'root' : [] | [string],
  'vertices' : Array<[string, EditorElement]>,
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
  'create_element_tree' : ActorMethod<[string], Result>,
  'create_file' : ActorMethod<[string], string>,
  'delete_file' : ActorMethod<[string], string>,
  'get_directories' : ActorMethod<[], [] | [FileDirectory]>,
  'get_directory' : ActorMethod<[string], [] | [FileDirectory]>,
  'get_element_trees' : ActorMethod<[], Result_1>,
  'get_file' : ActorMethod<[string], Result_2>,
  'get_profile' : ActorMethod<[], [] | [UserQuery]>,
  'get_users' : ActorMethod<[], Array<UserQuery>>,
  'group_update' : ActorMethod<[string], [] | [string]>,
  'register' : ActorMethod<[string], string>,
  'rename_file' : ActorMethod<[string], string>,
  'update_file' : ActorMethod<[string], string>,
  'update_profile' : ActorMethod<[string], string>,
}
