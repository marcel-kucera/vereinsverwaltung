import { API_URL } from "$lib/config";
import { CrudRepo } from "$lib/crudRepo.svelte";
import { okorerr } from "$lib/util";

export interface PaymentNew {
  year: number;
  memberid: number;
}

export type Payment = PaymentNew & { id: number };

const PAYMENT_URL = API_URL + "payment";

export async function getPayment(id: number): Promise<Payment[]> {
  return fetch(PAYMENT_URL + `?memberid=${id}`, { credentials: "include" })
    .then(okorerr)
    .then((res) => res.json());
}

export async function postPayment(payment: PaymentNew): Promise<Response> {
  return fetch(PAYMENT_URL, {
    method: "POST",
    body: JSON.stringify(payment),
    headers: { "Content-Type": "application/JSON" },
    credentials: "include",
  }).then(okorerr);
}

export async function deletePayment(id: number): Promise<Response> {
  return fetch(PAYMENT_URL + `?id=${id}`, {
    method: "DELETE",
    credentials: "include",
  }).then(okorerr);
}

export function makePaymentRepo(id: number) {
  return new CrudRepo({
    get: () => getPayment(id),
    add: (year: number) => postPayment({ memberid: id, year }),
    delete: deletePayment,
  });
}
