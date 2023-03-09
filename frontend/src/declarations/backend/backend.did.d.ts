import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface EditorElement {
  'id' : string,
  'tag' : [] | [string],
  'content' : string,
  'attrs' : Array<[string, string]>,
}
export interface EditorTree {
  'root' : [] | [string],
  'vertices' : Array<[string, EditorElement]>,
  'adjacency' : Array<[string, Array<string>]>,
}
export interface ElementTree { 'id' : string, 'elements' : EditorTree }
export interface FileDirectory {
  'id' : string,
  'files' : FileTree,
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
export interface FileTree {
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
export type getElementTreesResult = { 'Ok' : Array<[string, ElementTree]> } |
  { 'Err' : string };
export interface _SERVICE {
  'change_directory' : ActorMethod<[string], string>,
  'create_directory' : ActorMethod<[], string>,
  'create_element_tree' : ActorMethod<
    [string],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'create_file' : ActorMethod<[string], string>,
  'delete_file' : ActorMethod<[string], string>,
  'get_directories' : ActorMethod<[], [] | [FileDirectory]>,
  'get_directory' : ActorMethod<[string], [] | [FileDirectory]>,
  'get_element_trees' : ActorMethod<[], getElementTreesResult>,
  'get_file' : ActorMethod<
    [string],
    { 'Ok' : [FileNode, ElementTree] } |
      { 'Err' : string }
  >,
  'get_profile' : ActorMethod<[], [] | [UserQuery]>,
  'get_users' : ActorMethod<[], Array<UserQuery>>,
  'group_update' : ActorMethod<[string], [] | [string]>,
  'register' : ActorMethod<[string], string>,
  'rename_file' : ActorMethod<[string], string>,
  'update_file' : ActorMethod<[string], string>,
  'update_profile' : ActorMethod<[string], string>,
}
