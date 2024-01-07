<template>
  <rd-panel
    class="rd-panel"
    label="Daftar"
    :state="panelState"
    @exit="emits('exit')"
  >
    <div class="rd-panel-body">
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
          :input="usernameInput"
          @focusout="validate"
        />
      </div>
      <div class="rd-panel-input-wrapper">
        <rd-input-text
          class="rd-panel-input"
          :input="passwordInput"
          @focusout="validate"
        />
      </div>
    </div>
    <div class="rd-panel-footer">
      <rd-input-button
        class="rd-panel-button"
        label="daftar"
        :disabled="!name || !username || !password"
        :loading="submitLoading"
        @clicked="submit"
      />
    </div>
  </rd-panel>
</template>

<script lang="ts" setup>
  import { UserRequest } from "~/types/user";
  import { InputOption } from "~~/types/general";

  const props = defineProps<{
    state: "idle" | "hide";
  }>();
  const emits = defineEmits(["exit", "open-panel", "change-page", "shake"]);
  const { createUser } = useUser();

  const panelState = ref<"idle" | "hide">("idle");

  const nameInput = ref<InputOption<string>>({
    name: "name",
    label: "Name",
    placeholder: "Tekos Abady",
    model: {
      name: "",
      value: "",
    },
    error: "",
  });
  const usernameInput = ref<InputOption<string>>({
    name: "username",
    label: "Username",
    placeholder: "tekos",
    model: {
      name: "",
      value: "",
    },
    error: "",
  });
  const passwordInput = ref<InputOption<string>>({
    name: "password",
    label: "Pasword",
    placeholder: "Shushushsuhs",
    type: "password",
    model: {
      name: "",
      value: "",
    },
    error: "",
  });

  const submitLoading = ref<boolean>(false);

  const name = computed<UserRequest["name"]>(() => nameInput.value.model.value);
  const username = computed<UserRequest["username"]>(
    () => usernameInput.value.model.value
  );
  const password = computed<UserRequest["password"]>(
    () => passwordInput.value.model.value
  );

  function validate(input: any): void {
    if (!input.model?.value) input.error = "Kolom wajib diisi!";
    else input.error = "";
  }
  async function submit(): Promise<void> {
    submitLoading.value = true;

    const request: UserRequest = {
      name: name.value,
      username: username.value,
      password: password.value,
      role: "regular",
    };

    submitLoading.value = false;
    if (await createUser({ request })) {
      panelState.value = "hide";
    } else {
      emits("shake");
    }
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
