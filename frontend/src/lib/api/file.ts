import { API_URL } from "$lib/config";
import { CrudRepo } from "$lib/crudRepo.svelte";
import { okorerr } from "$lib/util";

export interface FileEntry {
  id: number;
  name: string;
}

const FILELIST_URL = API_URL + "filelist";

export async function getFileList(memberid: number) {
  return fetch(FILELIST_URL + `?memberid=${memberid}`, {
    credentials: "include",
  })
    .then(okorerr)
    .then((res) => res.json())
    .then((res: FileEntry[]) => {
      return res;
    });
}

const FILE_URL = API_URL + "file";

export function fileLink(id: number) {
  return FILE_URL + `?id=${id}`;
}

export async function uploadFile(memberid: number, file: File) {
  let data = new FormData();
  data.set("memberid", memberid + "");
  data.set("file", file);

  return fetch(FILE_URL, {
    method: "POST",
    body: data,
    credentials: "include",
  }).then(okorerr);
}

export async function deleteFile(fileid: number) {
  return fetch(FILE_URL + `?id=${fileid}`, {
    method: "DELETE",
    credentials: "include",
  }).then(okorerr);
}

export function makeFileRepo(id: number) {
  return new CrudRepo({
    get: () => getFileList(id),
    add: (file: File) => uploadFile(id, file),
    delete: deleteFile,
  });
}
