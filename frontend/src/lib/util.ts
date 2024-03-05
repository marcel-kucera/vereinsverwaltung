export function emptyPromise<T>(): Promise<T> {
  return new Promise<T>(() => {});
}

export async function okorerr(res: Response) {
  if (res.ok) {
    return res;
  } else {
    throw new Error(await res.text());
  }
}

export function unixToDatestring(t: number) {
  const date = new Date(t);
  const year = date.getFullYear();
  const month = ("0" + (date.getMonth() + 1)).slice(-2);
  const day = ("0" + date.getDate()).slice(-2);
  return year + "-" + month + "-" + day;
}
