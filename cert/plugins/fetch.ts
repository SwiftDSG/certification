import { User } from "~/types/user";

export default defineNuxtPlugin(() => {
  const { user } = useUser();
  const { public: pubConfig } = useRuntimeConfig();
  const atkCookie = useCookie<string>("atk", {
    maxAge: 1800,
  });
  const rtkCookie = useCookie<string>("rtk", {
    maxAge: 86400,
  });

  let config: RequestInit = {
    mode: "cors",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
  };
  return {
    provide: {
      setDefaults: (option: RequestInit): void => {
        config = {
          ...config,
          ...option,
          headers: { ...option.headers, "Content-Type": "application/json" },
        };
      },
      fetch: async (
        url: string,
        method: "get" | "post" | "delete" | "put" | "patch",
        body?: string | FormData
      ): Promise<Response> => {
        const option: RequestInit = {
          ...config,
          method,
          body,
        };
        if (body && body instanceof FormData && option.headers) {
          const headers = JSON.parse(JSON.stringify(option.headers));
          if (headers["Content-Type"]) delete headers["Content-Type"];
          option.headers = headers;
        }

        let response = await fetch(url, option);

        if (response.status === 401) {
          if (rtkCookie.value) {
            const refreshResponse = await fetch(
              `${pubConfig.apiBase}/users/refresh`,
              {
                ...config,
                method: "post",
                body: JSON.stringify({ rtk: rtkCookie.value }),
              }
            );
            const result: { atk: string; rtk: string; user: User } =
              await refreshResponse.json();
            if (result.atk && result.rtk) {
              atkCookie.value = result.atk;
              rtkCookie.value = result.rtk;
              config = {
                ...config,
                headers: {
                  Authorization: `Bearer ${result.atk}`,
                  "Content-Type": "application/json",
                },
              };
              user.value = result.user;
              if (user.value)
                response = await fetch(url, {
                  ...config,
                  method,
                  body,
                });
            }
          }
        }
        return response;
      },
    },
  };
});
