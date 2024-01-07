import { Loan, LoanRequest } from "~~/types/loan";
import useAlert from "./use-alert";

export default function () {
  const { setAlert } = useAlert();
  const { $fetch } = useNuxtApp();
  const { public: env } = useRuntimeConfig();

  const loans = useState<Loan[]>("loans", () => []);

  async function getLoan(): Promise<Loan[]> {
    try {
      const response = await $fetch(`${env.apiBase}/loans`, "get");

      if (response.status === 404) return [];
      if (response.status !== 200) throw await response.text();

      loans.value = await response.json();
      return loans.value;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menampilkan daftar pinjaman",
          message: e.message || "",
        });
      }
      return [];
    }
  }
  async function createLoan(payload: {
    request: LoanRequest;
  }): Promise<Loan | null> {
    try {
      const response = await $fetch(
        `${env.apiBase}/loans`,
        "post",
        JSON.stringify(payload.request)
      );

      if (response.status !== 201) throw await response.text();

      const result = await response.json();
      loans.value.unshift(result);

      return result;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menambahkan pinjaman buku",
          message: e.message || "",
        });
      }
      return null;
    }
  }
  async function updeteLoan(payload: {
    loan_id: string;
  }): Promise<Loan | null> {
    try {
      const response = await $fetch(
        `${env.apiBase}/loans/${payload.loan_id}`,
        "put"
      );

      if (response.status !== 200) throw await response.text();

      const result = await response.json();
      await getLoan();

      return result;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menyunting pinjaman",
          message: e.message || "",
        });
      }
      return null;
    }
  }
  async function deleteLoan(loan_id: string): Promise<void> {
    try {
      const response = await $fetch(
        `${env.apiBase}/loans/${loan_id}`,
        "delete"
      );

      if (response.status !== 204) throw await response.text();

      const index = loans.value.findIndex((a) => a._id === loan_id);
      if (index > -1) {
        loans.value.splice(index, 1);
      }
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menyunting pinjaman",
          message: e.message || "",
        });
      }
    }
  }

  return { loans, getLoan, createLoan, updeteLoan, deleteLoan };
}
