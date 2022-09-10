import { toast } from "solid-toast";

export const successToast = (message: string) => {
  toast.success(message, { position: "bottom-right" });
};

export const errorToast = (message: string) => {
  toast.error(message, { position: "bottom-right" });
};

export const commandWrapper =
  <T, U>(command: (arg: T) => Promise<U>) =>
  async (arg: T) => {
    try {
      const res = await command(arg);
      return res;
    } catch (e) {
      if (typeof e === "string") {
        errorToast(e);
      }
      throw e;
    }
  };
