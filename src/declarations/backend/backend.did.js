export const idlFactory = ({ IDL }) => {
  const CreateFileData = IDL.Record({
    'content' : IDL.Text,
    'name' : IDL.Text,
    'child_id' : IDL.Text,
    'parent_id' : IDL.Opt(IDL.Text),
  });
  const DeleteFileData = IDL.Record({
    'child_id' : IDL.Text,
    'parent_id' : IDL.Opt(IDL.Text),
  });
  const BackendError = IDL.Variant({ 'FileDoesNotExist' : IDL.Null });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : BackendError });
  const ReadFileData = IDL.Record({
    'child_id' : IDL.Text,
    'parent_id' : IDL.Opt(IDL.Text),
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : BackendError });
  const UpdateFileData = IDL.Record({
    'old_text' : IDL.Text,
    'child_id' : IDL.Text,
    'parent_id' : IDL.Opt(IDL.Text),
    'new_text' : IDL.Text,
  });
  return IDL.Service({
    'create_file' : IDL.Func([CreateFileData], [], []),
    'delete_file' : IDL.Func([DeleteFileData], [Result], []),
    'read_file' : IDL.Func([ReadFileData], [Result_1], ['query']),
    'read_files' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))))],
        ['query'],
      ),
    'update_file' : IDL.Func([UpdateFileData], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
