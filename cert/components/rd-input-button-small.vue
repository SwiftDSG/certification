<template>
  <button
    class="rd-input-component"
    ref="rdInputComponent"
    @mousedown="mouseDownHandler"
    @keydown.space.enter="keyDownHandler"
    :class="`${
      disabled ? 'rd-input-component-disabled' : ''
    } rd-input-component-${type || 'default'} ${
      side ? `rd-input-component-${side}` : ''
    }`"
    :disabled="disabled"
  >
    <div v-if="icon" class="rd-input-icon-container">
      <rd-svg
        :name="icon"
        :color="type === 'primary' ? 'secondary' : undefined"
      />
    </div>
    <div v-else-if="image" class="rd-input-image-container">
      <img :src="image" class="rd-input-image" />
    </div>
    <div v-else-if="color" class="rd-input-color-container">
      <div :style="`background: ${color}`" class="rd-input-color"></div>
    </div>
    <div class="rd-input-overlay"></div>
    <div class="rd-input-border"></div>
    <div class="rd-input-tooltip" v-if="tooltip">
      <span class="rd-input-tooltip-message rd-headline-6">{{ tooltip }}</span>
    </div>
  </button>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  const props = defineProps<{
    icon?: string;
    image?: string;
    color?: string;
    disabled?: boolean;
    type?: "default" | "primary" | "secondary";
    side?: "left" | "right";
    tooltip?: string;
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
        duration: 0.25,
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
  };

  function mouseDownHandler(e?: MouseEvent): MouseEvent | undefined {
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

  onBeforeUnmount(() => {
    window.removeEventListener("mouseup", releaseHandler);
    window.removeEventListener("keyup", releaseHandler);
  });
</script>

<style lang="scss" scoped>
  button.rd-input-component {
    cursor: pointer;
    position: relative;
    width: 2rem;
    height: 2rem;
    display: flex;
    border-radius: 0.5rem;
    margin: 0;
    background: var(--background-depth-one-color);
    border: none;
    padding: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    * {
      pointer-events: none;
    }
    .rd-input-icon-container,
    .rd-input-image-container,
    .rd-input-color-container {
      position: relative;
      width: 100%;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      img.rd-input-image {
        position: relative;
        width: 100%;
        height: 100%;
        border-radius: 0.5rem;
        object-fit: cover;
      }
      .rd-input-color {
        position: relative;
        width: 1.25rem;
        height: 1.25rem;
        border-radius: 0.375rem;
      }
    }
    .rd-input-icon-container {
      padding: 0 0.5rem;
      box-sizing: border-box;
    }
    .rd-input-overlay {
      pointer-events: none;
      position: absolute;
      top: 0;
      left: 0;
      border-radius: 0.5rem;
      width: 100%;
      height: 100%;
      background: rgba(0, 0, 0, 0.1);
      opacity: 0;
    }
    .rd-input-tooltip {
      z-index: 2000000;
      position: absolute;
      top: calc(100% + 0.25rem);
      height: 1rem;
      padding: 0 0.25rem;
      border-radius: 0.25rem;
      background: var(--font-primary-color);
      opacity: 0;
      transform: scale(0.75);
      transform-origin: top center;
      display: flex;
      justify-content: center;
      align-items: center;
      transition: 0.125s opacity, 0.125s transform ease-out;
      span.rd-input-tooltip-message {
        position: relative;
        color: var(--background-depth-one-color);
        white-space: nowrap;
      }

      &::after {
        content: " ";
        position: absolute;
        bottom: 100%;
        left: 50%;
        transform: translateX(-50%);
        border-width: 0.25rem;
        border-style: solid;
        border-color: transparent transparent var(--font-primary-color)
          transparent;
      }
    }
    .rd-input-border {
      pointer-events: none;
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      border-radius: 0.5rem;
      border: var(--border);
      box-sizing: border-box;
      transition: 0.25s border-color, 0.25s border-width;
      &::before {
        content: "";
        position: absolute;
        top: -3px;
        left: -3px;
        width: calc(100% + 6px);
        height: calc(100% + 6px);
        border-radius: 0.5rem;
        border: 3px solid var(--primary-color);
        box-sizing: border-box;
        opacity: 0;
        transition: 0.25s opacity;
      }
    }
    &.rd-input-component-disabled {
      pointer-events: none;
      filter: grayscale(0.75);
      opacity: 0.25;
    }
    &.rd-input-component-primary {
      background: var(--primary-color);
      border-color: var(--primary-color);
      .rd-input-border {
        border-color: var(--primary-color);
      }
    }
    &.rd-input-component-left {
      .rd-input-overlay {
        border-top-right-radius: 0;
        border-bottom-right-radius: 0;
      }
      .rd-input-border {
        border-right: 0.5px solid var(--border-color);
        border-top-right-radius: 0;
        border-bottom-right-radius: 0;
      }
    }
    &.rd-input-component-right {
      .rd-input-overlay {
        border-top-left-radius: 0;
        border-bottom-left-radius: 0;
      }
      .rd-input-border {
        border-left: 0.5px solid var(--border-color);
        border-top-left-radius: 0;
        border-bottom-left-radius: 0;
      }
    }
    &:hover {
      z-index: 10;
      &:not(:active) {
        .rd-input-tooltip {
          opacity: 1;
          transform: scale(1);
        }
      }
    }
    &:focus-visible {
      outline: none;
      .rd-input-border {
        border-color: var(--primary-color);
        &::before {
          opacity: 0.25;
        }
      }
    }
  }
</style>
