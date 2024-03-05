import { API_URL } from "$lib/config";
import { CrudRepo } from "$lib/crudRepo.svelte";
import { okorerr } from "$lib/util";

export interface MemberNew {
  firstname: string;
  lastname: string;
  email: string;
  plz: string;
  city: string;
  street: string;
  housenumber: string;
  iban: string;
  bic: string;
  sepa: boolean;
  note: string | undefined;
  joindate: number;
}

export type Member = MemberNew & { id: number; paid: boolean };

const MEMBER_URL = API_URL + "member";

export async function getMembers(): Promise<Member[]> {
  return fetch(MEMBER_URL, { credentials: "include" })
    .then(okorerr)
    .then((res) => res.json())
    .then((res: Member[]) => {
      res.forEach((m) => (m.joindate *= 1000));
      return res;
    });
}

export async function getMember(id: number): Promise<Member> {
  return fetch(MEMBER_URL + `?id=${id}`, { credentials: "include" })
    .then(okorerr)
    .then((res) => res.json())
    .then((res) => res[0])
    .then((res: Member) => {
      res.joindate *= 1000;
      return res;
    });
}

export async function postMember(member: MemberNew): Promise<Response> {
  member.joindate /= 1000;
  return fetch(MEMBER_URL, {
    method: "POST",
    body: JSON.stringify(member),
    headers: { "Content-Type": "application/JSON" },
    credentials: "include",
  }).then(okorerr);
}

export async function deleteMember(id: number): Promise<Response> {
  return fetch(MEMBER_URL + `?id=${id}`, {
    method: "DELETE",
    credentials: "include",
  }).then(okorerr);
}

export async function putMember(id: number, member: MemberNew) {
  member.joindate /= 1000;
  return fetch(MEMBER_URL + `?id=${id}`, {
    method: "PUT",
    body: JSON.stringify(member),
    headers: { "Content-Type": "application/JSON" },
    credentials: "include",
  }).then(okorerr);
}

export function makeMemberRepo() {
  return new CrudRepo({
    get: getMembers,
    select: getMember,
    add: postMember,
    update: putMember,
    delete: deleteMember,
  });
}
