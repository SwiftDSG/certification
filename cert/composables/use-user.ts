import { CookieRef } from "nuxt/dist/app/composables";
import { UserRequest, User } from "~~/types/user";
import useAlert from "./use-alert";

export default () => {
  const { setAlert } = useAlert();
  const { $setDefaults, $fetch } = useNuxtApp();
  const config = useRuntimeConfig();
  const atkCookie: CookieRef<string> = useCookie<string>("atk", {
    maxAge: 1800,
  });
  const rtkCookie: CookieRef<string> = useCookie<string>("rtk", {
    maxAge: 86400,
  });

  const user = useState<User | null>("user", () => null);
  const users = useState<User[] | null>("users", () => null);

  const getUser = async (payload: {
    user_id: string;
  }): Promise<User | null> => {
    try {
      const response: Response = await $fetch(
        `${config.public.apiBase}/users/${payload.user_id}`,
        "get"
      );
      if (response.status !== 200) throw new Error("");

      const result = await response.json();
      user.value = result;

      return result;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menampilkan pengguna",
          message: e.message || "",
        });
      }
      return null;
    }
  };
  const getUsers = async (): Promise<User[]> => {
    try {
      const response: Response = await $fetch(
        `${config.public.apiBase}/users`,
        "get"
      );
      if (response.status !== 200) throw new Error("");

      const result = await response.json();
      users.value = result;

      return result;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menampilkan daftar pengguna",
          message: e.message || "",
        });
      }
      return [];
    }
  };
  const updateUser = async (payload: {
    user_id: string;
    request: UserRequest;
  }): Promise<User | null> => {
    try {
      const response: Response = await $fetch(
        `${config.public.apiBase}/users/${payload.user_id}`,
        "put",
        JSON.stringify(payload.request)
      );
      if (response.status !== 200) throw new Error(await response.text());
      const result = await response.json();

      return result;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menyunting pengguna",
          message: e.message || "",
        });
      }
      return null;
    }
  };
  const createUser = async (payload: {
    request: UserRequest;
  }): Promise<User | null> => {
    try {
      const response: Response = await $fetch(
        `${config.public.apiBase}/users`,
        "post",
        JSON.stringify(payload.request)
      );
      if (response.status !== 201) throw new Error("");

      const result = await response.json();

      return result;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal membuat pengguna",
          message: e.message || "",
        });
      }
      return null;
    }
  };
  const login = async (
    username: string,
    password: string
  ): Promise<User | null> => {
    try {
      const response: Response = await $fetch(
        `${config.public.apiBase}/users/login`,
        "post",
        JSON.stringify({ username, password })
      );
      const result: { atk: string; rtk: string; user: User } =
        await response.json();
      if (!result.atk || !result.rtk) throw new Error("");
      atkCookie.value = result.atk;
      rtkCookie.value = result.rtk;
      $setDefaults({
        headers: {
          Authorization: `Bearer ${result.atk}`,
        },
      });
      user.value = result.user;
      return result.user;
    } catch {
      return null;
    }
  };
  const refresh = async (): Promise<User | null> => {
    try {
      if (!rtkCookie.value) throw new Error("COOKIE_UNAVAILABLE");
      const response: Response = await $fetch(
        `${config.public.apiBase}/users/refresh`,
        "post",
        JSON.stringify({ rtk: rtkCookie.value })
      );
      const result: { atk: string; rtk: string; user: User } =
        await response.json();
      if (!result.atk || !result.rtk) throw new Error("");
      atkCookie.value = result.atk;
      rtkCookie.value = result.rtk;
      $setDefaults({
        headers: {
          Authorization: `Bearer ${result.atk}`,
        },
      });
      user.value = result.user;
      return result.user;
    } catch (e) {
      logout();
      return null;
    }
  };
  const logout = (): void => {
    atkCookie.value = "";
    rtkCookie.value = "";
    user.value = null;
    $setDefaults({
      headers: {
        Authorization: "",
      },
    });
  };

  return {
    user,
    users,
    getUser,
    getUsers,
    createUser,
    updateUser,
    login,
    logout,
    refresh,
  };
};
