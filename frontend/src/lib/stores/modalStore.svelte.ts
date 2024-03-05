let title = $state("");
let content = $state("");
let callback = $state(() => {});
let open = $state(false);

export const modalStore = {
  get title() {
    return title;
  },
  get content() {
    return content;
  },
  get callback() {
    return callback;
  },
  get open() {
    return open;
  },
  ask(modalTitle: string, modalContent: string, confirmCallback: () => void) {
    title = modalTitle;
    content = modalContent;
    callback = confirmCallback;
    open = true;
  },
  close(){
    open = false;
  }
};
