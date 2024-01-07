<template>
  <div
    ref="rdComponent"
    class="rd-component"
    :class="`${type === 'overlay' ? 'rd-component-overlay' : ''} ${
      immediate && state === 'show' ? 'rd-component-immediate' : ''
    } ${fixed && type === 'overlay' ? 'rd-component-fixed' : ''}`"
  >
    <div class="rd-progress-bar-outer">
      <div class="rd-progress-bar-inner"></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  const props = defineProps<{
    state: "show" | "hide";
    type?: "default" | "overlay";
    immediate?: boolean;
    fixed?: boolean;
  }>();
  const rdComponent = ref<HTMLDivElement | null>(null);

  const animate = {
    init(rdComponent: HTMLElement): GSAPTimeline {
      const tl: GSAPTimeline = gsap.timeline();
      tl.to(rdComponent, {
        pointerEvents: "auto",
        opacity: 1,
        scale: 1,
        duration: 0.5,
        ease: "power2.inOut",
      });
      return tl;
    },
    exit(rdComponent: HTMLElement): void {
      const tl: GSAPTimeline = gsap.timeline();
      tl.to(rdComponent, {
        pointerEvents: "none",
        opacity: 0,
        scale: 1.125,
        duration: 0.5,
        ease: "power2.inOut",
      });
    },
  };

  watch(
    () => props.state,
    (val) => {
      if (rdComponent.value) {
        if (val === "hide") animate.exit(rdComponent.value);
        else animate.init(rdComponent.value);
      }
    }
  );

  onMounted(() => {
    setTimeout(() => {
      if (props.state === "show" && rdComponent.value && !props.immediate)
        animate.init(rdComponent.value);
    }, 250);
  });
</script>

<style lang="scss" scoped>
  .rd-component {
    pointer-events: none;
    position: relative;
    width: 10rem;
    height: 2rem;
    padding: 0.75rem;
    border-radius: 0.75rem;
    box-sizing: border-box;
    background: var(--background-depth-one-color);
    box-shadow: var(--box-shadow);
    opacity: 0;
    display: flex;
    justify-content: center !important;
    align-items: center !important;
    transform: scale(0.875);
    overflow: hidden;
    .rd-progress-bar-outer {
      position: relative;
      width: 100%;
      height: 0.25rem;
      border-radius: 0.125rem;
      background: var(--background-depth-three-color);
      overflow: hidden;
      display: flex;
      .rd-progress-bar-inner {
        position: absolute;
        top: 0;
        left: 0;
        right: 100%;
        height: 100%;
        background: var(--primary-color);
        border-radius: 0.125rem;
        animation: rd-loading 2s ease infinite;
      }
    }
    &.rd-component-overlay {
      position: absolute;
      top: 0;
      right: 0;
      bottom: 0;
      left: 0;
      width: 100%;
      height: 100%;
      padding: 0 2rem;
      border-radius: 0;
      background: var(--background-depth-one-color);
      opacity: 0;
      transform: scale(1) !important;
      .rd-progress-bar-outer {
        max-width: 15rem;
      }
      &.rd-component-immediate {
        opacity: 1;
      }
      &.rd-component-fixed {
        position: fixed;
      }
    }
  }
</style>
