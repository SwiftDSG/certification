<template>
  <div class="rd-layout" ref="rdLayout">
    <section class="rd-section">
      <header class="rd-header">
        <div class="rd-header-title-container">
          <span class="rd-header-title rd-headline-1"
            >Perpustakaan Baik Sentosa</span
          >
        </div>
        <div class="rd-header-filter-container">
          <div class="rd-header-filter">
            <rd-input-search
              :input="searchInput"
              class="rd-header-filter-input"
            />
          </div>
        </div>
        <div
          class="rd-header-progress-bar"
          :class="searchLoading ? 'rd-header-progress-bar-active' : ''"
        >
          <div class="rd-header-progress-bar-outer">
            <div class="rd-header-progress-bar-inner"></div>
          </div>
        </div>
      </header>
      <main class="rd-body">
        <rd-book
          v-for="book in books"
          :key="book._id"
          :book="book"
          @edit-book="
            panelHandler({
              state: 'show',
              type: 'book',
              data: {
                book,
              },
            })
          "
          @borrow-book="bookHandler"
          style="width: calc((100vw - 28rem) / 5)"
        />
      </main>
    </section>
    <section class="rd-section">
      <header v-if="user?.role === 'admin'" class="rd-header">
        <rd-input-button-small
          icon="plus"
          type="primary"
          tooltip="Tambah buku"
          @clicked="
            panelHandler({
              state: 'show',
              type: 'book',
            })
          "
        />
        <rd-input-button-small
          v-if="view !== 'desktop'"
          icon="dots"
          @clicked="
            panelHandler({
              state: 'show',
              type: 'menu',
            })
          "
        />
      </header>
      <div v-if="view === 'desktop'" class="rd-body">
        <div class="rd-menu" style="cursor: default">
          <div class="rd-menu-icon-container">
            <rd-svg class="rd-icon" name="weather-night" />
          </div>
          <div class="rd-menu-details-container">
            <span class="rd-menu-name rd-headline-5">Dark mode</span>
            <rd-input-toggle class="rd-menu-theme-switch" :input="themeInput" />
          </div>
        </div>
        <div
          v-if="!user"
          class="rd-menu"
          @click="
            panelHandler({
              state: 'show',
              type: 'user-login',
            })
          "
        >
          <div class="rd-menu-icon-container">
            <rd-svg class="rd-icon" name="login" />
          </div>
          <div class="rd-menu-details-container">
            <span class="rd-menu-name rd-headline-5">Login</span>
            <span class="rd-menu-description rd-caption-text"
              >Masuk ke aplikasi untuk fitur lengkap</span
            >
          </div>
        </div>
        <div
          v-else
          v-for="menu in menus.filter((a) =>
            a.role.includes(user?.role || 'regular')
          )"
          :key="menu.action"
          class="rd-menu"
          @click="clickHandler(menu.action)"
        >
          <div class="rd-menu-icon-container">
            <rd-svg class="rd-icon" :name="menu.icon" />
          </div>
          <div class="rd-menu-details-container">
            <span class="rd-menu-name rd-headline-5">{{ menu.name }}</span>
            <span class="rd-menu-description rd-caption-text">{{
              menu.description
            }}</span>
          </div>
        </div>
      </div>
    </section>
    <rd-user-login-panel
      v-if="panelOpened === 'user-login'"
      :state="panelState"
      :data="panelData[0]"
      @exit="panelHandler({ state: 'hide' })"
      @open-panel="panelHandler"
    />
    <rd-user-add-panel
      v-if="panelOpened === 'user-add'"
      :state="panelState"
      :data="panelData[0]"
      @exit="panelHandler({ state: 'hide' })"
      @open-panel="panelHandler"
      @shake="shake"
    />
    <rd-book-panel
      v-if="panelOpened === 'book'"
      :state="panelState"
      :data="panelData[0]"
      @exit="panelHandler({ state: 'hide' })"
      @open-panel="panelHandler"
      @shake="shake"
    />
    <rd-loan-panel
      v-if="panelOpened === 'loan'"
      :state="panelState"
      :data="panelData[0]"
      @exit="panelHandler({ state: 'hide' })"
      @open-panel="panelHandler"
      @shake="shake"
    />
    <rd-alert />
  </div>
</template>

<script lang="ts" setup>
  import { InputSearchOption, InputToggleOption } from "~~/types/general";
  import { UserRole } from "~~/types/user";

  type PanelHandlerOption = {
    state: "show" | "hide";
    type?: PanelType;
    data?: any;
  };
  type PanelType =
    | "book"
    | "loan"
    | "user"
    | "user-login"
    | "user-add"
    | "menu";
  type Menu = {
    action: "loan" | "user" | "logout";
    name: string;
    description: string;
    icon: string;
    role: UserRole[];
  };

  const { view, rem, theme, getTheme, setTheme } = useMain();
  const { logout, user, refresh } = useUser();
  const { books, getBook } = useBook();
  const { getLoan } = useLoan();

  const searchInput = ref<InputSearchOption>({
    name: "search",
    model: "",
    placeholder: "Cari buku...",
  });

  const rdLayout = ref<HTMLDivElement | undefined>(undefined);

  const menus = ref<Menu[]>([
    {
      action: "loan",
      name: "Daftar pinjaman",
      description: "Lihat atau ubah daftar pinjaman",
      icon: "list",
      role: ["admin"],
    },
    // {
    //   action: "user",
    //   name: "Daftar pengguna",
    //   description: "Lihat, tambah atau ubah daftar pengguna",
    //   icon: "account-group",
    //   role: ["admin"],
    // },
    {
      action: "logout",
      name: "Log out",
      description: "Keluar dari aplikasi",
      icon: "logout",
      role: ["admin", "regular"],
    },
  ]);

  const themeInput = ref<InputToggleOption>({
    model: false,
  });

  const panelState = ref<"idle" | "hide">("idle");
  const panelData = ref<any[]>([]);
  const panelOpened = ref<PanelType | undefined>(undefined);
  const panelSequence = ref<(PanelType | undefined)[]>([]);

  const searchLoading = ref<boolean>(true);
  const searchTimeout = ref<NodeJS.Timeout | null>(null);

  const search = computed<string>(() => searchInput.value.model);

  function clickHandler(action: Menu["action"]): void {
    if (action === "logout") {
      panelState.value = "hide";
      logout();
    } else {
      panelHandler({
        state: "show",
        type: action,
      });
    }
  }
  function panelHandler({ state, type, data }: PanelHandlerOption): void {
    if (state === "show") {
      if (panelSequence.value.length === 0) {
        panelState.value = "idle";
        panelSequence.value.unshift(type);
        panelData.value.unshift(data || {});
        panelOpened.value = panelSequence.value[0];
      } else {
        panelState.value = "hide";
        panelSequence.value.push(type);
        panelData.value.push(data || {});
      }
    } else {
      panelOpened.value = undefined;
      let sequence: PanelHandlerOption["type"] = undefined;
      let payload: PanelHandlerOption["data"] = null;
      if (panelState.value === "hide") {
        panelState.value = "idle";
        sequence = panelSequence.value.pop();
        payload = panelData.value.pop();
        if (sequence === panelSequence.value[0]) {
          panelData.value.splice(0, 1);
          panelSequence.value.splice(0, 1);
        }
      } else {
        panelState.value = "hide";
        panelData.value.splice(0, 1);
        panelSequence.value.splice(0, 1);
        sequence = panelSequence.value.pop();
        payload = panelData.value.pop();
      }
      if (sequence) {
        setTimeout(() => {
          panelState.value = "idle";
          panelSequence.value.unshift(sequence);
          panelData.value.unshift(payload || {});
          panelOpened.value = panelSequence.value[0];
        }, 50);
      }
    }
  }
  function bookHandler(): void {
    if (!user.value) {
      panelHandler({
        state: "show",
        type: "user-login",
      });
    }
  }
  function resizeHandler(e: MediaQueryList | MediaQueryListEvent): void {
    if (e.matches) view.value = "mobile";
    else view.value = "desktop";
    rem.value = parseInt(getComputedStyle?.(document.body)?.fontSize) || 24;
  }
  function shake(): void {
    if (rdLayout.value) rdLayout.value.classList.add("rd-layout-shake");
    setTimeout(() => {
      if (rdLayout.value) rdLayout.value.classList.remove("rd-layout-shake");
    }, 500);
  }

  watch(
    () => themeInput.value.model,
    (val) => {
      setTheme(val ? "dark" : "light");
      if (val) document.documentElement.classList.add("rd-dark");
      else document.documentElement.classList.remove("rd-dark");
    }
  );
  watch(
    () => search.value,
    (val) => {
      if (searchTimeout.value) clearTimeout(searchTimeout.value);
      searchTimeout.value = setTimeout(async () => {
        searchLoading.value = true;

        await getBook(val);

        setTimeout(() => {
          searchLoading.value = false;
        }, 1000);
      }, 500);
    }
  );

  onBeforeMount(() => {
    refresh();
    getTheme();
    themeInput.value.model = theme.value === "dark";
  });
  onMounted(async () => {
    await getBook();
    await getLoan();

    const mediaQuery: MediaQueryList = window.matchMedia("(max-width: 1024px)");
    mediaQuery.addEventListener("change", resizeHandler);
    resizeHandler(mediaQuery);

    document.documentElement.style.setProperty(
      "--vh",
      `${window.innerHeight * 0.01}px`
    );
    window.addEventListener("resize", () => {
      document.documentElement.style.setProperty(
        "--vh",
        `${window.innerHeight * 0.01}px`
      );
    });

    setTimeout(() => {
      searchLoading.value = false;
    }, 1000);
  });
</script>

<style lang="scss" scoped>
  .rd-layout {
    position: relative;
    width: 100%;
    min-height: 100vh;
    background: var(--background-depth-two-color);
    display: flex;

    .rd-loading {
      z-index: 2000;
    }

    section.rd-section {
      &:first-child {
        z-index: 0;
        position: relative;
        width: calc(100% - 20rem);
        display: flex;
        flex-direction: column;
        header.rd-header {
          z-index: 2;
          position: fixed;
          top: 0;
          left: 0;
          width: calc(100% - 20rem);
          height: 10rem;
          background: var(--background-depth-one-color);
          border-bottom: var(--border);
          padding: 2rem 2rem 1rem 2rem;
          box-sizing: border-box;
          display: flex;
          flex-direction: column;
          justify-content: space-between;
          .rd-header-title-container {
            position: relative;
            width: 100%;
            height: 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
          }
          .rd-header-filter-container {
            position: relative;
            width: 100%;
            height: 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            .rd-header-filter {
              position: relative;
              display: flex;
              align-items: center;
              gap: 0.75rem;
              .rd-header-filter-input {
                width: 10rem;
              }
            }
          }
          .rd-header-progress-bar {
            pointer-events: none;
            position: absolute;
            left: 0;
            bottom: 0;
            width: 100%;
            height: 0.125rem;
            opacity: 0;
            display: flex;
            justify-content: center !important;
            align-items: center !important;
            overflow: hidden;
            transform: scaleY(0);
            transition: 0.25s transform, 0.25s opacity;
            .rd-header-progress-bar-outer {
              position: relative;
              width: 100%;
              height: 0.125rem;
              background: var(--background-depth-three-color);
              overflow: hidden;
              display: flex;
              .rd-header-progress-bar-inner {
                position: absolute;
                top: 0;
                left: 0;
                right: 100%;
                height: 100%;
                background: var(--primary-color);
                animation: rd-loading 2s ease infinite;
              }
            }
            &.rd-header-progress-bar-active {
              transform: scaleY(1);
              opacity: 1;
            }
          }
        }
        main.rd-body {
          position: relative;
          width: 100%;
          padding: 11rem 2rem 2rem 2rem;
          box-sizing: border-box;
          display: flex;
          flex-wrap: wrap;
          gap: 1rem;
        }
      }

      &:not(:first-child) {
        z-index: 1;
        position: fixed;
        right: 0;
        top: 0;
        width: 20rem;
        height: 100%;
        background: var(--background-depth-one-color);
        border-left: var(--border);
        box-sizing: border-box;
        display: flex;
        flex-direction: column;

        header.rd-header {
          position: relative;
          width: 100%;
          height: 4rem;
          padding: 2rem 2rem 0 2rem;
          box-sizing: border-box;
          display: flex;
          flex-direction: row-reverse;
          justify-content: space-between;
        }

        .rd-body {
          position: relative;
          width: 100%;
          height: calc(100% - 4rem);
          padding: 2rem;
          box-sizing: border-box;
          display: flex;
          flex-direction: column;
          gap: 0.75rem;
          overflow-y: auto;

          .rd-menu {
            cursor: pointer;
            position: relative;
            width: 100%;
            padding: 0.5rem;
            border: var(--border);
            border-radius: 0.75rem;
            box-sizing: border-box;
            display: flex;
            align-items: center;
            gap: 0.5rem;

            .rd-menu-icon-container {
              position: relative;
              width: 2rem;
              height: 2rem;
              padding: 0 0.5rem;
              border-radius: 0.5rem;
              border: var(--border);
              box-sizing: border-box;
              display: flex;
              justify-content: center;
              align-items: center;
            }

            .rd-menu-details-container {
              position: relative;
              width: calc(100% - 2.5rem);
              height: 100%;
              display: flex;
              flex-direction: column;
              justify-content: center;
              gap: 0.25rem;
            }

            // &:last-child {
            //   cursor: default;
            //   position: fixed;
            //   left: 2rem;
            //   bottom: 2rem;
            //   width: 16rem;
            // }
            .rd-menu-theme-switch {
              position: absolute;
              right: 0;
            }
          }
        }
      }
    }

    &.rd-layout-shake {
      animation: rd-shake 0.25s infinite;
    }

    @media only screen and (max-width: 1300px) {
      section.rd-section {
        &:first-child {
          width: 100%;
        }

        &:not(:first-child) {
          right: -20rem;
        }
      }
    }

    @media only screen and (max-width: 1024px) {
      section.rd-section {
        &:not(:first-child) {
          z-index: 3;
          right: 0;
          width: 100%;
          height: 4rem;
          border: none;

          header.rd-header {
            height: 100%;
            padding: 1rem;
          }
        }
      }
    }
  }
</style>

<style lang="scss">
  :root {
    -webkit-tap-highlight-color: transparent;
    --primary-color: #ffd975;
    --secondary-color: #242529;
    --error-color: #ff584c;
    --warning-color: #ffd975;
    --success-color: #6bc785;
    --border-color: #f0f0f0;
    --font-primary-color: #242529;
    --font-secondary-color: #242529;
    --font-sub-color: rgba(36, 37, 41, 0.5);
    --background-depth-one-color: #ffffff;
    --background-depth-two-color: #fafafa;
    --background-depth-three-color: #f0f0f0;
    --border: 1px solid var(--border-color);
    --box-shadow: 0 0.5rem 1rem rgba(199, 199, 199, 0.125);
  }

  ::-webkit-scrollbar {
    display: none;
  }

  html,
  body {
    position: relative;
    width: 100vw;
    margin: 0;
    padding: 0;
    font-size: 24px;
    font-family: "Poppins", -apple-system, BlinkMacSystemFont, sans-serif;
    color: var(--font-primary-color);
    background: var(--background-depth-two-color);
    overflow-x: hidden;

    @media only screen and (max-width: 1900px) and (min-width: 1600px) {
      font-size: 22px;
    }

    @media only screen and (max-width: 1599px) and (min-width: 1480px) {
      font-size: 21px;
    }

    @media only screen and (max-width: 1479px) and (min-width: 1380px) {
      font-size: 20px;
    }

    @media only screen and (max-width: 1379px) and (min-width: 1320px) {
      font-size: 19px;
    }

    @media only screen and (max-width: 1319px) and (min-width: 1024px) {
      font-size: 18px;
    }

    @media only screen and (max-width: 640px) {
      font-size: 24px;
    }

    @media only screen and (max-width: 320px) {
      font-size: 17px;
    }

    @media only screen and (max-width: 1024px) {
      height: auto;
      overflow-y: auto;

      .rd-title-1 {
        font-size: 1.25rem;
      }

      .rd-title-2 {
        font-size: 1.125rem;
      }
    }

    &.rd-dark {
      --primary-color: #fff37a;
      --warning-color: #fff37a;
      --background-depth-one-color: #2d2e32;
      --background-depth-two-color: #242529;
      --background-depth-three-color: #202124;
      --border-color: #222327;
      --box-shadow: 0 0.5rem 1rem rgba(15, 16, 17, 0.25);
      --font-primary-color: #fdebdd;
      --font-secondary-color: #242529;
      --font-sub-color: rgba(253, 235, 221, 0.375);
    }
  }

  .rd-title-1 {
    font-size: 1.75rem;
    font-weight: 700;
    font-family: "Poppins";
  }

  .rd-title-2 {
    font-size: 1.375rem;
    font-weight: 700;
    font-family: "Poppins";
  }

  .rd-headline-1 {
    font-size: 1.25rem;
    font-weight: 600;
    font-family: "Poppins";
  }

  .rd-headline-2 {
    font-size: 1rem;
    font-weight: 600;
    font-family: "Poppins";
  }

  .rd-headline-3 {
    font-size: 0.85rem;
    font-weight: 600;
    font-family: "Poppins";
  }

  .rd-headline-4 {
    font-size: 0.75rem;
    font-weight: 600;
    font-family: "Poppins";
  }

  .rd-headline-5 {
    font-size: 0.65rem;
    font-weight: 600;
    font-family: "Poppins";
  }

  .rd-headline-6 {
    font-size: 0.55rem;
    font-weight: 600;
    font-family: "Poppins";
  }

  .rd-subtitle-text {
    font-size: 0.75rem;
    font-weight: 500;
    font-family: "Poppins";
  }

  .rd-body-text {
    font-size: 0.6rem;
    font-weight: 500;
    font-family: "Poppins";
  }

  .rd-caption-text {
    font-size: 0.55rem;
    font-family: "Poppins";
    font-weight: 500;
    color: var(--font-sub-color);
  }

  .rd-button-text {
    font-family: "Poppins";
    font-size: 0.55rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.125rem;
    transform: translateX(0.0625rem);
  }

  span.rd-text-wrapper,
  span.rd-word-wrapper,
  span.rd-letter-wrapper,
  span.rd-image-wrapper {
    position: relative;
    overflow: hidden;
    display: flex;
    justify-content: center;
    align-items: flex-start;

    span.rd-text-container,
    span.rd-word-container,
    span.rd-letter-container,
    span.rd-image-container {
      position: relative;
      overflow: hidden;
      display: flex;
      justify-content: center;
      align-items: center;

      &.rd-text-container-up,
      &.rd-word-container-up,
      &.rd-letter-container-up,
      &.rd-image-container-up {
        transform: translateY(-100%);

        span.rd-text,
        span.rd-word,
        span.rd-letter,
        img.rd-image {
          transform: translateY(100%);

          &.rd-image:not(.rd-image-contain) {
            transform: translateY(100%) scale(1.25);
          }

          &.rd-image-contain {
            object-fit: contain;
            transform: translateY(100%) scale(1);
          }
        }
      }

      &.rd-text-container-down,
      &.rd-word-container-down,
      &.rd-letter-container-down,
      &.rd-image-container-down {
        transform: translateY(100%);

        span.rd-text,
        span.rd-word,
        span.rd-letter,
        img.rd-image {
          transform: translateY(-100%);

          &.rd-image:not(.rd-image-contain) {
            transform: translateY(-100%) scale(1.25);
          }

          &.rd-image-contain {
            object-fit: contain;
            transform: translateY(-100%) scale(1);
          }
        }
      }

      &.rd-text-container-left,
      &.rd-word-container-left,
      &.rd-letter-container-left,
      &.rd-image-container-left {
        transform: translateX(-100%);

        span.rd-text,
        span.rd-word,
        span.rd-letter,
        img.rd-image {
          transform: translateX(100%);

          &.rd-image:not(.rd-image-contain) {
            transform: translateX(100%) scale(1.25);
          }

          &.rd-image-contain {
            object-fit: contain;
            transform: translateX(100%) scale(1);
          }
        }
      }

      &.rd-text-container-right,
      &.rd-word-container-right,
      &.rd-letter-container-right,
      &.rd-image-container-right {
        transform: translateX(100%);

        span.rd-text,
        span.rd-word,
        span.rd-letter,
        img.rd-image {
          transform: translateX(-100%);

          &.rd-image:not(.rd-image-contain) {
            transform: translateX(-100%) scale(1.25);
          }

          &.rd-image-contain {
            object-fit: contain;
            transform: translateX(-100%) scale(1);
          }
        }
      }
    }
  }

  span.rd-image-wrapper {
    width: 100%;
    height: 100%;

    span.rd-image-container {
      width: 100%;
      height: 100%;

      img.rd-image {
        position: relative;
        width: 100%;
        height: 100%;
        object-fit: cover;
        transform: scale(1.25);
      }
    }
  }

  h1,
  h2,
  h3,
  h4,
  h5,
  h6,
  p {
    margin: 0;
    padding: 0;
  }

  @keyframes rd-loading {
    0% {
      left: 0;
      right: 100%;
    }

    50% {
      left: 0;
      right: 0;
    }

    100% {
      left: 100%;
      right: 0;
    }
  }

  @keyframes rd-shake {
    0% {
      transform: translate(1px, 1px);
    }

    10% {
      transform: translate(-1px, -2px);
    }

    20% {
      transform: translate(-3px, 0px);
    }

    30% {
      transform: translate(3px, 2px);
    }

    40% {
      transform: translate(1px, -1px);
    }

    50% {
      transform: translate(-1px, 2px);
    }

    60% {
      transform: translate(-3px, 1px);
    }

    70% {
      transform: translate(3px, 1px);
    }

    80% {
      transform: translate(-1px, -1px);
    }

    90% {
      transform: translate(1px, 2px);
    }

    100% {
      transform: translate(1px, -2px);
    }
  }

  @keyframes rd-rotate {
    0% {
      transform: rotate(0deg);
    }

    100% {
      transform: rotate(360deg);
    }
  }

  @keyframes rd-circular-rotate {
    0% {
      transform: rotate(0);
    }

    50% {
      transform: rotate(-140deg);
    }

    100% {
      transform: rotate(0);
    }
  }
</style>
