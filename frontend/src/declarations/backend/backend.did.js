export const idlFactory = ({ IDL }) => {
  const FileMode = IDL.Variant({
    'Private' : IDL.Null,
    'Public' : IDL.Null,
    'Restricted' : IDL.Null,
  });
  const FileNode = IDL.Record({
    'id' : IDL.Text,
    'name' : IDL.Text,
    'file_mode' : FileMode,
    'element_tree' : IDL.Opt(IDL.Text),
  });
  const FileTree = IDL.Record({
    'root' : IDL.Opt(IDL.Text),
    'vertices' : IDL.Vec(IDL.Tuple(IDL.Text, FileNode)),
    'adjacency' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Text))),
  });
  const FileDirectory = IDL.Record({
    'id' : IDL.Text,
    'files' : FileTree,
    'name' : IDL.Text,
  });
  const EditorElement = IDL.Record({
    'id' : IDL.Text,
    'tag' : IDL.Opt(IDL.Text),
    'content' : IDL.Text,
    'attrs' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const EditorTree = IDL.Record({
    'root' : IDL.Opt(IDL.Text),
    'vertices' : IDL.Vec(IDL.Tuple(IDL.Text, EditorElement)),
    'adjacency' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Text))),
  });
  const ElementTree = IDL.Record({ 'id' : IDL.Text, 'elements' : EditorTree });
  const getElementTreesResult = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Text, ElementTree)),
    'Err' : IDL.Text,
  });
  const UserQuery = IDL.Record({
    'username' : IDL.Opt(IDL.Text),
    'birthdate' : IDL.Opt(IDL.Text),
    'emails' : IDL.Opt(IDL.Vec(IDL.Text)),
    'email' : IDL.Opt(IDL.Text),
    'address' : IDL.Text,
    'first_name' : IDL.Opt(IDL.Text),
    'last_name' : IDL.Opt(IDL.Text),
    'image' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  return IDL.Service({
    'change_directory' : IDL.Func([IDL.Text], [IDL.Text], []),
    'create_directory' : IDL.Func([], [IDL.Text], []),
    'create_element_tree' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'create_file' : IDL.Func([IDL.Text], [IDL.Text], []),
    'delete_file' : IDL.Func([IDL.Text], [IDL.Text], []),
    'get_directories' : IDL.Func([], [IDL.Opt(FileDirectory)], ['query']),
    'get_directory' : IDL.Func([IDL.Text], [IDL.Opt(FileDirectory)], ['query']),
    'get_element_trees' : IDL.Func([], [getElementTreesResult], ['query']),
    'get_file' : IDL.Func(
        [IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Tuple(FileNode, ElementTree),
            'Err' : IDL.Text,
          }),
        ],
        ['query'],
      ),
    'get_profile' : IDL.Func([], [IDL.Opt(UserQuery)], ['query']),
    'get_users' : IDL.Func([], [IDL.Vec(UserQuery)], ['query']),
    'group_update' : IDL.Func([IDL.Text], [IDL.Opt(IDL.Text)], []),
    'register' : IDL.Func([IDL.Text], [IDL.Text], []),
    'rename_file' : IDL.Func([IDL.Text], [IDL.Text], []),
    'update_file' : IDL.Func([IDL.Text], [IDL.Text], []),
    'update_profile' : IDL.Func([IDL.Text], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
