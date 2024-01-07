<template>
  <rd-panel
    class="rd-panel"
    :label="data.book ? 'Ubah buku' : 'Tambah buku'"
    :state="panelState"
    :action="data.book ? 'delete' : ''"
    @clicked="remove"
    @exit="emits('exit')"
  >
    <div class="rd-panel-body">
      <div class="rd-panel-input-wrapper" style="margin-bottom: 1rem">
        <rd-input-image class="rd-panel-input" :input="imageInput" />
      </div>
      <div class="rd-panel-input-wrapper">
        <rd-input-text
          class="rd-panel-input"
          :input="nameInput"
          @focusout="validate"
        />
      </div>
      <div class="rd-panel-input-wrapper">
        <rd-input-text
          class="rd-panel-input"
          :input="authorInput"
          @focusout="validate"
        />
      </div>
      <div class="rd-panel-input-wrapper">
        <rd-input-text
          class="rd-panel-input"
          :input="yearInput"
          @focusout="validate"
        />
      </div>
      <div class="rd-panel-input-wrapper" style="margin-bottom: 6rem">
        <rd-input-textarea
          class="rd-panel-input"
          :input="descriptionInput"
          @focusout="validate"
        />
      </div>
    </div>
    <div class="rd-panel-footer">
      <rd-input-button
        class="rd-panel-button"
        label="daftar"
        :disabled="!name || !author || !year"
        :loading="submitLoading"
        @clicked="submit"
      />
    </div>
  </rd-panel>
</template>

<script lang="ts" setup>
  import { Book, BookRequest } from "~/types/book";
  import { InputFileOption, InputOption } from "~~/types/general";

  const props = defineProps<{
    state: "idle" | "hide";
    data: {
      book?: Book;
    };
  }>();
  const emits = defineEmits(["exit", "open-panel", "change-page", "shake"]);
  const { createBook, updeteBookImage, updeteBook, deleteBook } = useBook();
  const { public: env } = useRuntimeConfig();

  const panelState = ref<"idle" | "hide">("idle");

  const imageInput = ref<InputFileOption>({
    label: "Cover buku",
    type: "image",
    file: undefined,
  });
  const nameInput = ref<InputOption<string>>({
    name: "name",
    label: "Nama",
    placeholder: "Tekos Abady",
    model: {
      name: "",
      value: "",
    },
    error: "",
  });
  const authorInput = ref<InputOption<string>>({
    name: "author",
    label: "Penulis",
    placeholder: "Tekos Semen Tyara",
    model: {
      name: "",
      value: "",
    },
    error: "",
  });
  const yearInput = ref<InputOption<number>>({
    name: "year",
    label: "Tahun rilis",
    placeholder: "2093",
    model: {
      name: "",
      value: 2000,
    },
    error: "",
    type: "number",
  });
  const descriptionInput = ref<InputOption<string>>({
    name: "description",
    label: "Sinopsis",
    placeholder: "Lalalalala",
    model: {
      name: "",
      value: "",
    },
    error: "",
  });

  const submitLoading = ref<boolean>(false);

  const image = computed<File | undefined>(() => imageInput.value.file);
  const name = computed<BookRequest["name"]>(() => nameInput.value.model.value);
  const author = computed<BookRequest["author"]>(
    () => authorInput.value.model.value
  );
  const year = computed<BookRequest["year"]>(() =>
    parseInt(yearInput.value.model.name.split(".").join(""))
  );
  const description = computed<BookRequest["description"]>(
    () => descriptionInput.value.model.name
  );

  function validate(input: any): void {
    if (!input.model?.value) input.error = "Kolom wajib diisi!";
    else input.error = "";
  }
  async function remove(): Promise<void> {
    if (props.data.book) {
      submitLoading.value = true;

      await deleteBook(props.data.book._id);

      submitLoading.value = false;
      panelState.value = "hide";
    }
  }
  async function submit(): Promise<void> {
    submitLoading.value = true;

    const request: BookRequest = {
      name: name.value,
      author: author.value,
      year: year.value,
      description: description.value,
    };

    let book: Book | null = null;

    if (props.data.book) {
      book = await updeteBook({ book_id: props.data.book._id, request });
    } else {
      book = await createBook({
        request,
      });
    }

    if (image.value && book) {
      await updeteBookImage({ book_id: book._id, request: image.value });
      submitLoading.value = false;
      panelState.value = "hide";
    } else {
      submitLoading.value = false;
      if (book) {
        panelState.value = "hide";
      } else {
        emits("shake");
      }
    }
  }

  watch(
    () => props.state,
    (val) => {
      if (val === "hide") panelState.value = "hide";
    }
  );

  onMounted(() => {
    if (props.data.book) {
      nameInput.value.model = {
        name: props.data.book.name,
        value: props.data.book.name,
      };
      authorInput.value.model = {
        name: props.data.book.author,
        value: props.data.book.author,
      };
      yearInput.value.model = {
        name: props.data.book.year.toString(),
        value: props.data.book.year,
      };
      if (props.data.book.description) {
        descriptionInput.value.model = {
          name: props.data.book.description,
          value: props.data.book.description,
        };
      }
      if (props.data.book.media) {
        imageInput.value.image_url = `${env.apiBase}/medias?_id=${props.data.book.media.id}&extension=${props.data.book.media.ext}`;
      }
    }
  });
</script>

<style lang="scss" scoped>
  .rd-panel {
    .rd-panel-body {
      position: relative;
      width: 100%;
      display: flex;
      flex-direction: column;

      .rd-panel-input-wrapper {
        position: relative;
        width: 100%;
        display: flex;
        gap: 0.5rem;

        .rd-panel-input {
          position: relative;
          width: 100%;
        }
      }
      p.rd-panel-message {
        position: relative;
        width: 100%;
        span.rd-panel-message-highlight {
          cursor: pointer;
          position: relative;
          color: var(--primary-color);
        }
      }
    }

    .rd-panel-footer {
      position: fixed;
      bottom: 0;
      left: 0;
      width: 100%;
      height: 6rem;
      background: var(--background-depth-one-color);
      padding: 2rem;
      box-sizing: border-box;
      display: flex;
      justify-content: center;
      align-items: center;

      .rd-panel-button {
        width: 100%;
      }

      &::after {
        content: "";
        position: absolute;
        bottom: 100%;
        left: 0;
        width: 100%;
        height: 1px;
        background: var(--border-color);
      }
    }

    @media only screen and (max-width: 1024px) {
      .rd-panel-footer {
        height: 4rem;
        padding: 1rem;
      }
    }
  }
</style>
