import { API_URL } from "$lib/config";
import { okorerr } from "$lib/util";

export async function getLoginStatus(): Promise<boolean> {
  return fetch(API_URL + "status", { credentials: "include" }).then(
    async (res) => {
      switch (res.status) {
        case 200: {
          return true;
        }
        case 401: {
          return false;
        }
        default: {
          throw new Error(await res.text());
        }
      }
    },
  );
}

export interface Login {
  name: string;
  password: string;
}

export async function login(login: Login): Promise<Response> {
  return fetch(API_URL + "login", {
    body: JSON.stringify(login),
    method: "POST",
    headers: { "Content-Type": "application/JSON" },
    credentials: "include",
  }).then(okorerr);
}

export async function logout(): Promise<Response> {
  return fetch(API_URL + "logout", {
    credentials: "include",
  }).then(okorerr);
}
