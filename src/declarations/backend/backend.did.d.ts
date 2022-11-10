import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type BackendError = { 'FileDoesNotExist' : null };
export interface CreateFileData {
  'content' : string,
  'name' : string,
  'child_id' : string,
  'parent_id' : [] | [string],
}
export interface DeleteFileData {
  'child_id' : string,
  'parent_id' : [] | [string],
}
export interface ReadFileData {
  'child_id' : string,
  'parent_id' : [] | [string],
}
export type Result = { 'Ok' : null } |
  { 'Err' : BackendError };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : BackendError };
export interface UpdateFileData {
  'old_text' : string,
  'child_id' : string,
  'parent_id' : [] | [string],
  'new_text' : string,
}
export interface _SERVICE {
  'create_file' : ActorMethod<[CreateFileData], undefined>,
  'delete_file' : ActorMethod<[DeleteFileData], Result>,
  'read_file' : ActorMethod<[ReadFileData], Result_1>,
  'read_files' : ActorMethod<[], Array<[string, Array<[string, string]>]>>,
  'update_file' : ActorMethod<[UpdateFileData], Result>,
}
