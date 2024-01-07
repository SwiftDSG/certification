<template>
  <button
    class="rd-input-component"
    ref="rdInputComponent"
    :class="type === 'secondary' ? 'rd-input-secondary' : ''"
    :aria-label="label"
    @mousedown="mouseDownHandler"
    @keydown.space.enter="keyDownHandler"
  >
    <div class="rd-input-ripple-container">
      <div class="rd-input-ripple"></div>
    </div>
    <div class="rd-input-label-container">
      <label
        class="rd-input-label rd-button-text rd-input-label-main"
        :data-label="label"
      >
        {{ label }}
      </label>
    </div>
    <div class="rd-input-border"></div>
    <div class="rd-input-overlay"></div>
  </button>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  const props = defineProps<{
    label: string;
    state: "uninitialized" | "initialized";
    type?: "primary" | "secondary";
  }>();
  const emits = defineEmits(["clicked"]);

  const rdInputComponent = ref<HTMLButtonElement | null>(null);

  const buttonAnimating = ref<boolean>(false);
  const buttonClicking = ref<boolean>(false);
  const buttonPressed = ref<boolean>(false);

  const animate = {
    click(rdInputComponent: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdOverlay: HTMLElement | null =
        rdInputComponent.querySelector(".rd-input-overlay");

      tl.to(rdInputComponent, {
        scale: 0.95,
        duration: 0.25,
        ease: "power2.out",
      }).to(
        rdOverlay,
        {
          opacity: 1,
          duration: 0.25,
          ease: "power2.inOut",
        },
        "<0"
      );
    },
    release(rdInputComponent: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdOverlay: HTMLElement | null =
        rdInputComponent.querySelector(".rd-input-overlay");

      tl.to(rdInputComponent, {
        scale: 1,
        duration: 0.5,
        ease: "back.out(2)",
      }).to(
        rdOverlay,
        {
          opacity: 0,
          duration: 0.25,
          ease: "power2.out",
        },
        "<0"
      );
    },
    init(rdInputComponent: HTMLElement): void {
      const tl: GSAPTimeline = gsap.timeline();

      tl.to(rdInputComponent, {
        opacity: 1,
        duration: 0.25,
        delay: 0.75,
        ease: "power0",
      });
    },
    exit(rdInputComponent: HTMLElement): void {
      const tl: GSAPTimeline = gsap.timeline();

      tl.to(rdInputComponent, {
        opacity: 0,
        duration: 0.25,
        delay: 0.25,
        ease: "power0",
      });
    },
  };

  function mouseDownHandler(e: MouseEvent): MouseEvent {
    buttonAnimating.value = true;
    buttonClicking.value = true;
    buttonPressed.value = true;
    if (rdInputComponent.value) {
      animate.click(rdInputComponent.value, () => {
        buttonClicking.value = false;
        if (!buttonPressed.value) {
          releaseHandler();
        }
      });
    }
    window.addEventListener("mouseup", releaseHandler);
    return e;
  }
  function keyDownHandler(e?: KeyboardEvent): KeyboardEvent | undefined {
    buttonAnimating.value = true;
    buttonClicking.value = true;
    buttonPressed.value = true;
    if (rdInputComponent.value) {
      animate.click(rdInputComponent.value, () => {
        buttonClicking.value = false;
        if (!buttonPressed.value) {
          releaseHandler();
        }
      });
    }
    window.addEventListener("keyup", releaseHandler);

    return e;
  }
  function releaseHandler(): void {
    window.removeEventListener("mouseup", releaseHandler);
    window.removeEventListener("keyup", releaseHandler);
    buttonPressed.value = false;
    if (!buttonClicking.value && rdInputComponent.value) {
      emits("clicked");
      animate.release(rdInputComponent.value, () => {
        buttonAnimating.value = false;
      });
    }
  }

  watch(
    () => props.state,
    (val) => {
      if (val === "initialized" && rdInputComponent.value) {
        animate.init(rdInputComponent.value);
      } else if (rdInputComponent.value) {
        animate.exit(rdInputComponent.value);
      }
    }
  );

  onBeforeUnmount(() => {
    window.removeEventListener("mouseup", releaseHandler);
    window.removeEventListener("keyup", releaseHandler);
  });

  onMounted(() => {
    if (props.state === "initialized" && rdInputComponent.value) {
      animate.init(rdInputComponent.value);
    }
  });
</script>

<style lang="scss" scoped>
  button.rd-input-component {
    cursor: pointer;
    position: relative;
    height: 3rem;
    border-radius: 1.5rem;
    background: transparent;
    border: none;
    padding: 0;
    margin: 0;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    opacity: 0;
    * {
      pointer-events: none;
    }
    .rd-input-ripple-container {
      z-index: 1;
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      border-radius: inherit;
      transform: translateZ(0);
      overflow: hidden;
      .rd-input-ripple {
        position: relative;
        width: 100%;
        height: 100%;
        background: var(--primary-color);
        border-radius: 50% 50% 0 0;
        transform: translateY(101%);
        transition: transform 0.5s cubic-bezier(0.4, 0, 0, 1),
          border-radius 0.5s cubic-bezier(0.4, 0, 0, 1);
      }
    }
    .rd-input-label-container {
      z-index: 2;
      position: relative;
      width: 100%;
      height: 100%;
      padding: 0 1.25rem;
      box-sizing: border-box;
      display: flex;
      justify-content: center;
      align-items: center;
      overflow: hidden;
      label.rd-input-label {
        position: relative;
        width: 100%;
        height: 100%;
        color: var(--primary-color);
        text-transform: uppercase;
        white-space: nowrap;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: transform 0.8s cubic-bezier(0.16, 1, 0.3, 1);
        &::after {
          content: attr(data-label);
          position: absolute;
          width: 100%;
          height: 100%;
          top: 110%;
          left: 0;
          display: flex;
          justify-content: center;
          align-items: center;
          color: var(--font-primary-color);
        }
      }
    }
    .rd-input-border {
      pointer-events: none;
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      border-radius: 1.5rem;
      border: 1px solid var(--primary-color);
      box-sizing: border-box;
      opacity: 0.5;
      transition: 0.25s border-color, 0.25s border-width, 0.25s opacity;
      &::before {
        content: "";
        position: absolute;
        top: -3px;
        left: -3px;
        width: calc(100% + 6px);
        height: calc(100% + 6px);
        border-radius: calc(1.5rem + 3px);
        border: 3px solid var(--primary-color);
        box-sizing: border-box;
        opacity: 0;
        transition: 0.25s opacity;
      }
    }
    .rd-input-overlay {
      pointer-events: none;
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background: rgba(0, 0, 0, 0.05);
      border-radius: 1.5rem;
      opacity: 0;
    }
    &.rd-input-secondary {
      .rd-input-border {
        background: var(--font-primary-color);
        border-color: var(--font-primary-color);
      }
    }
    &:hover {
      .rd-input-ripple-container {
        .rd-input-ripple {
          border-radius: 0;
          transform: translateY(0);
          transition-duration: 0.5s, 0.9s;
        }
      }
      .rd-input-label-container {
        label.rd-input-label {
          transform: translateY(-110%);
        }
      }
    }
    &:focus-visible {
      outline: none;
      .rd-input-border {
        border-color: var(--primary-color);
        opacity: 1;
        &::before {
          opacity: 0.25;
        }
      }
    }
  }
</style>
