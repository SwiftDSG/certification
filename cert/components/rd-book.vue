<template>
  <div class="rd-book">
    <div v-if="book.media" class="rd-book-image-container">
      <img
        class="rd-book-image"
        :src="`${env.apiBase}/medias?_id=${book.media.id}&extension=${book.media.ext}`"
        alt=""
        srcset=""
      />
    </div>
    <div class="rd-book-container">
      <div
        v-if="loan"
        class="rd-book-warning"
        :style="
          new Date().getTime() <= loan.borrow_date + 604800000
            ? ''
            : 'background: var(--error-color)'
        "
      >
        <div class="rd-book-warning-icon-container">
          <rd-svg
            class="rd-book-warning-icon"
            color="secondary"
            name="information"
          />
        </div>
        <span class="rd-book-warning-message rd-headline-6">{{
          new Date().getTime() <= loan.borrow_date + 604800000
            ? `Batas ${daysHanlder(
                loan.borrow_date + 604800000,
                loan.borrow_date
              )} hari`
            : `Telat ${daysHanlder(
                new Date().getTime(),
                loan.borrow_date + 604800000
              )} hari`
        }}</span>
      </div>
      <span class="rd-book-name rd-headline-5">{{ book.name }}</span>
      <div class="rd-book-detail-container">
        <span class="rd-book-detail rd-caption-text">{{ book.author }}</span>
        <div class="rd-book-detail-divider"></div>
        <span class="rd-book-detail rd-caption-text">{{ book.year }}</span>
      </div>
      <div class="rd-book-action-container">
        <rd-input-button
          :label="state === 'returnable' ? 'Kembalikan' : 'Pinjam'"
          class="rd-book-action"
          :style="
            user?.role === 'admin'
              ? 'width: calc(100% - 2.5rem)'
              : 'width: 100%'
          "
          :disabled="state === 'not-available'"
          :loading="loading"
          @clicked="bookHandler"
        />
        <rd-input-button-small
          v-if="user?.role === 'admin'"
          icon="pencil"
          :disabled="loading"
          @clicked="emits('edit-book')"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import { Book } from "~/types/book";
  import { Loan, LoanRequest } from "~/types/loan";

  const props = defineProps<{
    book: Book;
  }>();
  const { public: env } = useRuntimeConfig();
  const emits = defineEmits(["edit-book", "borrow-book"]);
  const { user } = useUser();
  const { loans, createLoan, updeteLoan } = useLoan();

  const loading = ref<boolean>(false);

  const loan = computed<Loan | undefined>(() => {
    if (!loans.value.length) return undefined;
    return loans.value.find(
      (a) =>
        a.book._id === props.book._id &&
        a.user._id === user.value?._id &&
        a.status === "unreturned"
    );
  });
  const state = computed<"available" | "returnable" | "not-available">(() => {
    if (loan.value) return "returnable";
    const borrowed = loans.value.filter((a) => a.status === "unreturned");
    if (borrowed.findIndex((a) => a.book._id === props.book._id) > -1)
      return "not-available";
    return "available";
  });

  async function bookHandler(): Promise<void> {
    if (loan.value) {
      loading.value = true;
      await updeteLoan({
        loan_id: loan.value._id,
      });
      loading.value = false;
    } else if (state.value === "available") {
      if (!user.value) {
        emits("borrow-book");
      } else {
        const request: LoanRequest = {
          book_id: props.book._id,
        };
        loading.value = true;
        await createLoan({
          request,
        });
        loading.value = false;
      }
    }
  }
  function daysHanlder(x: number, y: number) {
    return Math.floor((x - y) / 86400000);
  }
</script>

<style lang="scss" scoped>
  .rd-book {
    position: relative;
    aspect-ratio: 1 / 1.5;
    padding: 0.75rem;
    border-radius: 0.75rem;
    box-sizing: border-box;
    overflow: hidden;
    .rd-book-image-container {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      img.rd-book-image {
        position: relative;
        width: 100%;
        height: 100%;
        object-fit: cover;
      }
      &::after {
        content: "";
        position: absolute;
        width: 100%;
        height: 100%;
        background: linear-gradient(rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.5));
      }
    }
    .rd-book-container {
      position: relative;
      width: 100%;
      height: 100%;
      display: flex;
      flex-direction: column;
      justify-content: flex-end;
      .rd-book-warning {
        position: absolute;
        top: 0;
        height: 1.5rem;
        border-radius: 0.75rem;
        padding: 0.25rem 0.5rem 0.25rem 0.25rem;
        background: var(--primary-color);
        box-sizing: border-box;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        .rd-book-warning-icon-container {
          position: relative;
          width: 1rem;
          height: 1rem;
          border-radius: 50%;
          background: rgba(0, 0, 0, 0.125);
        }
        span.rd-book-warning-message {
          position: relative;
          color: var(--secondary-color);
        }
      }
      .rd-book-action-container {
        position: relative;
        width: 100%;
        margin-top: 0.5rem;
        display: flex;
        gap: 0.5rem;
      }
      .rd-book-name {
        cursor: pointer;
        color: #fff;
      }
      .rd-book-detail-container {
        position: relative;
        width: 100%;
        display: flex;
        align-items: center;
        gap: 0.25rem;
        * {
          color: #fff;
        }
        .rd-book-detail-divider {
          position: relative;
          width: 0.125rem;
          height: 0.125rem;
          border-radius: 50%;
          background: var(--font-primary-color);
        }
      }
    }
  }
</style>
