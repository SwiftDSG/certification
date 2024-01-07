<template>
  <rd-panel
    class="rd-panel"
    label="Daftar pinjaman"
    :state="panelState"
    @exit="emits('exit')"
  >
    <div class="rd-panel-body">
      <div v-for="loan in loans" :key="loan._id" class="rd-panel-loan">
        <div class="rd-panel-loan-header">
          <span class="rd-panel-loan-name rd-headline-5">{{
            loan.book.name
          }}</span>
          <rd-input-button-small
            icon="pencil"
            tooltip="Kembalikan"
            @clicked="forceReturn(loan._id)"
            :type="loan.status === 'returned' ? 'default' : 'primary'"
            :disabled="loan.status === 'returned'"
          />
        </div>
        <div class="rd-panel-loan-body">
          <div class="rd-panel-loan-data">
            <span class="rd-panel-loan-data-placeholder rd-caption-text"
              >Peminjam</span
            >
            <span class="rd-panel-loan-data-value rd-headline-6">{{
              loan.user.name
            }}</span>
          </div>
          <div class="rd-panel-loan-data">
            <span class="rd-panel-loan-data-placeholder rd-caption-text"
              >Status</span
            >
            <span class="rd-panel-loan-data-value rd-headline-6">{{
              loan.status === "returned" ? "Dikembalikan" : "Dipinjam"
            }}</span>
          </div>
          <div class="rd-panel-loan-data">
            <span class="rd-panel-loan-data-placeholder rd-caption-text"
              >Tanggal pinjam</span
            >
            <span class="rd-panel-loan-data-value rd-headline-6">{{
              $formatDateTime(loan.borrow_date)
            }}</span>
          </div>
          <div class="rd-panel-loan-data">
            <span class="rd-panel-loan-data-placeholder rd-caption-text">{{
              loan.status === "returned" ? "Dikembalikan" : "Batas kembali"
            }}</span>
            <span class="rd-panel-loan-data-value rd-headline-6">{{
              $formatDateTime(loan.return_date || loan.borrow_date + 604800000)
            }}</span>
          </div>
        </div>
      </div>
    </div>
  </rd-panel>
</template>

<script lang="ts" setup>
  const props = defineProps<{
    state: "idle" | "hide";
  }>();
  const emits = defineEmits(["exit", "open-panel", "change-page", "shake"]);
  const { loans, updeteLoan } = useLoan();
  const { $formatDateTime } = useNuxtApp();

  const panelState = ref<"idle" | "hide">("idle");

  async function forceReturn(loan_id: string): Promise<void> {
    await updeteLoan({ loan_id });
  }

  watch(
    () => props.state,
    (val) => {
      if (val === "hide") panelState.value = "hide";
    }
  );
</script>

<style lang="scss" scoped>
  .rd-panel {
    .rd-panel-body {
      position: relative;
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 1rem;
      .rd-panel-loan {
        position: relative;
        width: 100%;
        padding: 0.75rem;
        border: var(--border);
        border-radius: 0.75rem;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        .rd-panel-loan-header {
          position: relative;
          width: 100%;
          display: flex;
          justify-content: space-between;
          align-items: center;
        }
        .rd-panel-loan-body {
          position: relative;
          width: 100%;
          display: flex;
          flex-wrap: wrap;
          gap: 0.5rem 0;
          .rd-panel-loan-data {
            position: relative;
            width: 50%;
            display: flex;
            flex-direction: column;
            gap: 0.25rem;
          }
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
