import { Book, BookRequest } from "~~/types/book";
import useAlert from "./use-alert";

export default function () {
  const { setAlert } = useAlert();
  const { $fetch } = useNuxtApp();
  const { public: env } = useRuntimeConfig();

  const books = useState<Book[]>("books", () => []);

  async function getBook(text?: string): Promise<Book[]> {
    try {
      const response = await $fetch(
        `${env.apiBase}/books?${text ? `text=${text}` : ""}`,
        "get"
      );

      if (response.status === 404) return [];
      if (response.status !== 200) throw await response.text();

      books.value = await response.json();
      return books.value;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menampilkan koleksi buku",
          message: e.message || "",
        });
      }
      return [];
    }
  }
  async function createBook(payload: {
    request: BookRequest;
  }): Promise<Book | null> {
    try {
      const response = await $fetch(
        `${env.apiBase}/books`,
        "post",
        JSON.stringify(payload.request)
      );

      if (response.status !== 201) throw await response.text();

      const result = await response.json();
      books.value.unshift(result);

      return result;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menambahkan buku",
          message: e.message || "",
        });
      }
      return null;
    }
  }
  async function updeteBook(payload: {
    book_id: string;
    request: BookRequest;
  }): Promise<Book | null> {
    try {
      const response = await $fetch(
        `${env.apiBase}/books/${payload.book_id}`,
        "put",
        JSON.stringify(payload.request)
      );

      if (response.status !== 200) throw await response.text();

      const result = await response.json();
      await getBook();

      return result;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menyunting buku",
          message: e.message || "",
        });
      }
      return null;
    }
  }
  async function updeteBookImage(payload: {
    book_id: string;
    request: File;
  }): Promise<Book | null> {
    try {
      const data = new FormData();
      data.append("file", payload.request, payload.request.name);
      const response: Response = await $fetch(
        `${env.apiBase}/books/${payload.book_id}/image`,
        "put",
        data
      );
      if (response.status !== 200) throw new Error(await response.json());

      const result = await response.json();
      await getBook();

      return result;
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menyunting buku",
          message: e.message || "",
        });
      }
      return null;
    }
  }
  async function deleteBook(book_id: string): Promise<void> {
    try {
      const response = await $fetch(
        `${env.apiBase}/books/${book_id}`,
        "delete"
      );

      if (response.status !== 200) throw await response.text();

      const index = books.value.findIndex((a) => a._id === book_id);
      if (index > -1) {
        books.value.splice(index, 1);
      }
    } catch (e) {
      if (e instanceof Error) {
        setAlert({
          type: "error",
          title: "Gagal menghapus buku",
          message: e.message || "",
        });
      }
    }
  }

  return {
    books,
    getBook,
    createBook,
    updeteBookImage,
    updeteBook,
    deleteBook,
  };
}
