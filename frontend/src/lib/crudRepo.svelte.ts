import { emptyPromise } from "$lib/util";

type Data<T> = Promise<T[]>;
type Select<T> = Promise<T>;
type ModifiedPromise<_T> = Promise<Response>;

export interface CrudOptions<T, TNew> {
  get?: () => Data<T>;
  select?: (id: number) => Select<T>;
  add: (n: TNew) => ModifiedPromise<T>;
  update?: (id: number, n: TNew) => ModifiedPromise<T>;
  delete: (id: number) => ModifiedPromise<T>;
}

export class CrudRepo<T, TNew> {
  data: Data<T> = $state(emptyPromise());
  selected: Select<T> = $state(emptyPromise());

  addPromise?: ModifiedPromise<T> = $state();
  addInflight: boolean = $state(false);

  updatePromise?: ModifiedPromise<T> = $state();
  updateId?: number = $state();
  updateInflight: boolean = $state(false);

  deletePromise?: ModifiedPromise<T> = $state();
  deleteId?: number = $state();
  deleteInflight: boolean = $state(false);

  opts: CrudOptions<T, TNew>;

  private selectedId?: number;
  private getCalled: boolean;

  constructor(opts: CrudOptions<T, TNew>) {
    this.opts = opts;
    this.getCalled = false;
  }

  refresh() {
    let self = this;
    if (self.opts.get && self.getCalled) {
      let promise = self.opts.get();
      let swapper = () => (self.data = promise);
      promise.then(swapper, swapper);
    }
    if (self.opts.select && self.selectedId) {
      let promise = self.opts.select(self.selectedId);
      let swapper = () => (self.selected = promise);
      promise.then(swapper, swapper);
    }
  }

  get() {
    if (!this.opts.get) {
      throw "get() was not defined for this repo";
    }
    this.data = this.opts.get();
    this.getCalled = true;
  }

  select(id: number) {
    if (!this.opts.select) {
      throw "select() was not defined for this repo";
    }
    this.selected = this.opts.select(id);
  }

  add(n: TNew) {
    let self = this;

    this.addInflight = true;
    this.addPromise = this.opts.add(n);
    this.addPromise.then(() => self.refresh());
    this.addPromise.finally(() => (self.addInflight = false));
  }

  update(id: number, n: TNew) {
    if (!this.opts.update) {
      throw "update() was not defined for this repo";
    }
    let self = this;

    this.updateInflight = true;
    this.updatePromise = this.opts.update(id, n);
    this.updateId = id;
    this.updatePromise.then(() => self.refresh());
    this.updatePromise.finally(() => (self.updateId = undefined));
    this.updatePromise.finally(() => (self.updateInflight = false));
  }

  delete(id: number) {
    let self = this;

    this.deleteInflight = true;
    this.deletePromise = this.opts.delete(id);
    this.deleteId = id;
    this.deletePromise.then(() => self.refresh());
    this.deletePromise.finally(() => (self.deleteId = undefined));
    this.deletePromise.finally(() => (self.deleteInflight = false));
  }
}
