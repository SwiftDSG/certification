import { Alert } from "~~/types/alert";

type ErrorKind =
  | "SCALE_EMPTY"
  | "LOCATION_ALREADY_EXIST"
  | "LOCATION_CREATION_FAILED"
  | "INVALID_URL";

export default () => {
  const alert = useState<Alert | null>("alert", () => null);

  const setError = (e: any): void => {
    if (e && e.message && typeof e.message === "string") {
      const kind: ErrorKind = e.message;
      let title = "";
      let message = "";
      switch (kind) {
        case "SCALE_EMPTY":
          title = "Data kosong";
          message = "Silahkan mulai mengisi data";
          break;
        case "LOCATION_ALREADY_EXIST":
          title = "Nama lokasi sudah digunakan";
          message = "Silahkan menggunakan nama lain";
          break;
        case "LOCATION_CREATION_FAILED":
          title = "Pembuatan lokasi gagal";
          message = "Silahkan mencoba lagi dalam beberapa saat";
          break;
        case "INVALID_URL":
          title = "URL Salah";
          message = "Pastikan URL yang digunakan benar";
          break;
        default:
          title = kind;
          message = "Silahkan mencoba lagi dalam beberapa saat";
          break;
      }

      setAlert({
        type: "error",
        title,
        message,
      });
    } else {
      setAlert({
        type: "error",
        title: "Unknown error occured",
        message: "Please try again later",
      });
    }
  };
  const setAlert = (data: Alert): void => {
    alert.value = data;
  };

  const removeAlert = (): void => {
    alert.value = null;
  };

  return { alert, setError, setAlert, removeAlert };
};
