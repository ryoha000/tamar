import { toast } from "solid-toast";

export const successToast = (message: string) => {
  toast.success(message, { position: "bottom-right" });
};

export const errorToast = (message: string) => {
  toast.error(message, { position: "bottom-right" });
};

export const commandInitialValueWrapper =
  <T, U, V>(command: (arg: T) => Promise<U>, initialValue: V) =>
  async (arg: T) => {
    try {
      const res = await command(arg);
      return res;
    } catch (e) {
      if (typeof e === "string") {
        errorToast(e);
      }
      return initialValue;
    }
  };

export const commandNullWrapper =
  <T, U>(command: (arg: T) => Promise<U>) =>
  async (arg: T): Promise<U | null> => {
    return await commandInitialValueWrapper(command, null)(arg);
  };

export const commandArrayWrapper =
  <T, U>(command: (arg: T) => Promise<U[]>) =>
  async (arg: T): Promise<U[]> => {
    return await commandInitialValueWrapper(command, [])(arg);
  };
