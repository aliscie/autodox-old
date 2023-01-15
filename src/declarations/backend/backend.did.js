export const idlFactory = ({ IDL }) => {
  const Status = IDL.Variant({
    'InvalidInput' : IDL.Null,
    'Success' : IDL.Null,
    'UnAuthorized' : IDL.Null,
  });
  const UpdateRespone = IDL.Record({ 'status' : Status, 'message' : IDL.Text });
  const CreateFileData = IDL.Record({
    'id' : IDL.Text,
    'name' : IDL.Text,
    'children' : IDL.Opt(IDL.Vec(IDL.Text)),
    'parent_id' : IDL.Text,
    'directory_id' : IDL.Text,
  });
  const FileNode = IDL.Record({
    'id' : IDL.Text,
    'name' : IDL.Text,
    'element_tree' : IDL.Opt(IDL.Text),
  });
  const Tree = IDL.Record({
    'root' : IDL.Opt(IDL.Text),
    'vertices' : IDL.Vec(IDL.Tuple(IDL.Text, FileNode)),
    'adjacency' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Vec(IDL.Nat8)))),
  });
  const FileDirectory = IDL.Record({
    'id' : IDL.Text,
    'files' : Tree,
    'name' : IDL.Text,
  });
  const QueryUser = IDL.Opt(
    IDL.Record({
      'username' : IDL.Opt(IDL.Text),
      'image' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    })
  );
  const User = IDL.Record({
    'username' : IDL.Text,
    'image' : IDL.Vec(IDL.Nat8),
  });
  return IDL.Service({
    'create_directory' : IDL.Func([], [UpdateRespone], []),
    'create_file' : IDL.Func([CreateFileData], [], []),
    'get_directories' : IDL.Func([], [IDL.Vec(FileDirectory)], ['query']),
    'get_profile' : IDL.Func([], [QueryUser], ['query']),
    'register' : IDL.Func([IDL.Text], [UpdateRespone], []),
    'test_ic' : IDL.Func([], [IDL.Text], ['query']),
    'update_profile' : IDL.Func([User], [UpdateRespone], []),
  });
};
export const init = ({ IDL }) => { return []; };
