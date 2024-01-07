<template>
  <div class="rd-component" ref="rdComponent">
    <div class="rd-background" ref="rdBackground"></div>
    <div class="rd-panel" ref="rdPanel">
      <div
        v-if="label"
        class="rd-panel-header"
        :class="headerActive ? 'rd-panel-header-active' : ''"
      >
        <rd-input-button-small
          class="rd-panel-button"
          :icon="'arrow-left'"
          @clicked="exit"
        />
        <h2 v-if="label" class="rd-panel-title rd-headline-5">{{ label }}</h2>
        <rd-input-button-small
          v-if="action"
          class="rd-panel-button"
          :icon="action"
          :disabled="disabled"
          :type="type || 'primary'"
          @clicked="emits('clicked')"
        />
      </div>
      <div
        class="rd-panel-body"
        @scroll="scrollHandler"
        :class="!label ? 'rd-panel-body-full' : ''"
      >
        <h2 v-if="label" class="rd-panel-title rd-headline-3">{{ label }}</h2>
        <div class="rd-panel-content">
          <slot />
        </div>
      </div>
      <rd-progress-bar
        :state="loading ? 'show' : 'hide'"
        :immediate="true"
        class="rd-panel-overlay-loading"
        type="overlay"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";
  import { ComputedRef } from "vue";
  import { View } from "~~/types/general";

  const { view, rem } = useMain();
  const props = defineProps<{
    label?: string;
    action?: string;
    type?: "primary" | "secondary" | "default";
    disabled?: boolean;
    state: "idle" | "hide";
    loading?: boolean;
  }>();
  const emits = defineEmits(["exit", "clicked"]);

  const rdBackground = ref<HTMLDivElement | null>(null);
  const rdPanel = ref<HTMLDivElement | null>(null);

  const scrollValue = ref<number>(0);
  const scrollThreshold = ref<number>(0);

  const headerActive: ComputedRef<boolean> = computed(
    (): boolean => scrollValue.value >= scrollThreshold.value
  );

  const animate = {
    init(rdBackground: HTMLElement, rdPanel: HTMLElement): void {
      const tl: GSAPTimeline = gsap.timeline();

      tl.to(rdBackground, {
        opacity: 1,
        duration: 0.5,
      }).to(
        rdPanel,
        {
          x: 0,
          y: 0,
          duration: 0.5,
          ease: "power4.out",
        },
        "<0"
      );
    },
    exit(
      view: View,
      rdBackground: HTMLElement,
      rdPanel: HTMLElement,
      cb?: () => void
    ): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      tl.to(rdBackground, {
        opacity: 0,
        duration: 0.5,
      });

      if (view === "mobile") {
        tl.to(
          rdPanel,
          {
            y: "125%",
            duration: 0.5,
            ease: "power2.inOut",
          },
          "<0"
        );
      } else {
        tl.to(
          rdPanel,
          {
            x: "125%",
            duration: 0.5,
            ease: "power2.inOut",
          },
          "<0"
        );
      }
    },
  };

  function scrollHandler(e: UIEvent): UIEvent {
    if (e.target instanceof HTMLElement) {
      scrollValue.value = e.target.scrollTop;
    }
    return e;
  }

  function exit(): void {
    if (rdBackground.value && rdPanel.value) {
      animate.exit(view.value, rdBackground.value, rdPanel.value, () => {
        emits("exit");
      });
    }
  }

  watch(
    () => props.state,
    (val) => {
      if (val === "hide") exit();
    }
  );

  onMounted(() => {
    scrollThreshold.value = 2 * rem.value;
    if (rdBackground.value && rdPanel.value) {
      animate.init(rdBackground.value, rdPanel.value);
    }
  });
</script>

<style lang="scss" scoped>
  .rd-component {
    z-index: 100000;
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    height: calc(var(--vh) * 100);
    display: flex;
    .rd-background {
      position: absolute;
      top: 0;
      left: 0;
      width: 100vw;
      height: 100vh;
      background: rgba(43, 25, 6, 0.5);
      backdrop-filter: blur(10px);
      opacity: 0;
    }
    .rd-panel {
      position: absolute;
      top: 0;
      left: 0;
      width: 100vw;
      height: 100vh;
      height: calc(var(--vh) * 100);
      background: var(--background-depth-one-color);
      overflow-y: hidden;
      overflow-x: visible;
      display: flex;
      flex-direction: column;
      transform: translateY(125%);
      .rd-panel-header {
        z-index: 2;
        position: relative;
        width: 100%;
        height: 4rem;
        background: var(--background-depth-one-color);
        padding: 1rem;
        box-sizing: border-box;
        box-shadow: 0 0.5rem 0.5rem 0.5rem var(--background-depth-one-color);
        display: flex;
        flex-shrink: 0;
        justify-content: center;
        align-items: center;
        .rd-panel-button {
          position: absolute;
          top: 1rem;
          left: 1rem;
          &:last-child {
            left: auto;
            right: 1rem;
          }
        }
        .rd-panel-title {
          position: relative;
          opacity: 0;
          transform: scale(0.875);
          transition: 0.25s opacity, 0.25s transform;
        }
        &.rd-panel-header-active {
          box-shadow: none;
          .rd-panel-title {
            opacity: 1;
            transform: scale(1);
          }
          &::after {
            opacity: 1;
          }
        }
        &::after {
          content: "";
          position: absolute;
          top: 100%;
          left: 0;
          width: 100%;
          height: 1px;
          background: var(--border-color);
          opacity: 0;
          transition: 0.25s opacity;
        }
      }
      .rd-panel-body {
        z-index: 1;
        position: relative;
        width: 100%;
        height: calc(100% - 4rem);
        padding: 1rem 1rem 0 1rem;
        box-sizing: border-box;
        overflow-y: auto;
        overflow-x: hidden;
        display: flex;
        flex-shrink: 0;
        flex-direction: column;
        h2.rd-panel-title {
          position: relative;
          width: 100%;
          height: 1rem;
          margin-bottom: 1rem;
          display: flex;
          flex-shrink: 0;
          align-items: center;
        }
        .rd-panel-content {
          position: relative;
          width: 100%;
          flex-shrink: 0;
          display: flex;
          flex-direction: column;
        }
        &.rd-panel-body-full {
          height: 100%;
        }
      }
      .rd-panel-overlay {
        pointer-events: none;
        z-index: 999999;
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        border-radius: 1rem;
        opacity: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: 0.25s opacity;
        .rd-panel-overlay-loading {
          z-index: 2;
        }
        &::before {
          z-index: 0;
          content: "";
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          backdrop-filter: blur(5px);
        }
        &::after {
          z-index: 1;
          content: "";
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          background: var(--font-primary-color);
          opacity: 0.25;
        }
        &.rd-panel-overlay-active {
          pointer-events: all;
          opacity: 1;
        }
      }
    }
    @media only screen and (min-width: 1025px) {
      .rd-panel {
        left: auto;
        top: 0;
        right: 0;
        width: 20rem;
        box-sizing: border-box;
        transform: translateX(125%);
        .rd-panel-header {
          height: 6rem;
          padding: 2rem;
          .rd-panel-button {
            top: 2rem;
            left: 2rem;
            &:last-child {
              left: auto;
              right: 2rem;
            }
          }
        }
        .rd-panel-body {
          padding: 1rem 2rem;
          height: calc(100% - 6rem);
        }
      }
    }
  }
</style>
