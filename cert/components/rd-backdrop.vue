<template>
  <div class="rd-component" ref="rdComponent">
    <div class="rd-background" ref="rdBackground"></div>
    <div class="rd-panel" ref="rdPanel" @touchstart="dragStart">
      <div class="rd-panel-header">
        <rd-input-button-small
          class="rd-panel-button"
          :icon="'arrow-left'"
          @clicked="exit"
        />
      </div>
      <div class="rd-panel-body">
        <slot />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";
  import { View } from "types/general";
  import { ComputedRef } from "vue";

  const { view } = useMain();
  const emits = defineEmits(["exit"]);
  const props = defineProps<{
    state: "idle" | "hide";
  }>();

  const rdBackground = ref<HTMLDivElement | null>(null);
  const rdPanel = ref<HTMLDivElement | null>(null);

  let animationDY: number = 0;
  let animationTY: number = 0;
  let animationSliding: boolean = false;
  let animationTimedOut: boolean = false;
  let animationTimeout: NodeJS.Timeout | null = null;
  let animationInterval: NodeJS.Timer | null = null;
  let animationEventCurr: TouchEvent | null = null;
  let animationEventLast: TouchEvent | null = null;
  let animationSnapPos: number = 0;
  let animationSnapPosDist: number = 0;
  let animationVelocity: number = 0;

  const animationStart: ComputedRef<number> = computed(() => {
    return view.value === "mobile"
      ? window.innerHeight - animationTrack.value
      : 0;
  });
  const animationTrack: ComputedRef<number> = computed(() => {
    return rdPanel.value?.getBoundingClientRect().height || 0;
  });

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

  function exit(): void {
    if (rdBackground.value && rdPanel.value)
      animate.exit(view.value, rdBackground.value, rdPanel.value, () => {
        emits("exit");
      });
  }

  function dragStart(e: TouchEvent): void {
    if (view.value === "mobile") {
      animationTY = 0;
      animationSliding = true;
      animationTimedOut = false;
      animationTimeout = setTimeout(() => {
        animationTimedOut = true;
      }, 500);

      document.documentElement.addEventListener("touchmove", dragHandler);
      document.documentElement.addEventListener("touchend", dragEnd);

      animationInterval = setInterval(() => {
        if (animationEventLast && animationEventCurr && rdPanel.value) {
          const { y }: DOMRect = rdPanel.value.getBoundingClientRect();

          animationDY =
            animationEventCurr.touches[0].clientY -
            animationEventLast.touches[0].clientY;
          animationTY += animationDY;

          animationVelocity = animationDY * 60;
          if (Math.abs(animationVelocity) > 3600)
            animationVelocity = animationVelocity < 0 ? -3600 : 3600;

          if (y <= animationStart.value && animationVelocity < 0)
            animationVelocity *= 0.25;

          const percentage: number =
            (animationTrack.value - y + animationStart.value) /
            animationTrack.value;

          gsap.to(rdPanel.value, {
            y: `+=${animationVelocity / 60}`,
            duration: 0,
          });
          gsap.to(rdBackground.value, {
            opacity: `${percentage * 100}%`,
            duration: 0,
          });
        }
        animationEventLast = animationEventCurr;
      }, 50 / 3);
    }
  }
  function dragEnd(e: TouchEvent): void {
    animationSliding = false;
    animationEventCurr = null;
    animationEventLast = null;

    if (animationInterval) clearInterval(animationInterval);
    if (animationTimeout) clearTimeout(animationTimeout);

    document.documentElement.removeEventListener("touchmove", dragHandler);
    document.documentElement.removeEventListener("touchend", dragEnd);

    releaseHandler();
  }
  function dragHandler(e: TouchEvent): void {
    animationEventCurr = e;
  }

  function releaseHandler(): void {
    if (!animationSliding && animationTY && rdPanel.value) {
      const { y }: DOMRect = rdPanel.value.getBoundingClientRect();
      if (!animationSnapPos) {
        if (!animationTimedOut) {
          if (animationTY <= 0) {
            animationSnapPos = animationStart.value;
          } else {
            animationSnapPos = window.innerHeight;
          }
        } else {
          if ((y - animationStart.value) / animationTrack.value < 0.5) {
            animationSnapPos = animationStart.value;
          } else {
            animationSnapPos = window.innerHeight;
          }
        }
        animationSnapPosDist = animationSnapPos - y;
      }

      if (Math.abs(animationSnapPosDist) > 0.1) {
        requestAnimationFrame(releaseHandler);
        animationSnapPosDist = animationSnapPos - y;
        animationDY = animationSnapPosDist * 0.1;
        gsap.to(rdPanel.value, {
          y: `+=${animationDY}`,
          duration: 0,
        });
        const percentage: number =
          (animationTrack.value - y + animationStart.value) /
          animationTrack.value;
        gsap.to(rdBackground.value, {
          opacity: `${percentage * 100}%`,
          duration: 0,
        });
      } else {
        gsap.to(rdPanel.value, {
          y: `+=${animationSnapPosDist}`,
          duration: 0,
        });
        gsap.to(rdBackground.value, {
          opacity: animationSnapPos === animationStart.value ? 1 : 0,
          duration: 0,
        });

        if (animationSnapPos === window.innerHeight) {
          emits("exit");
        }

        animationSnapPos = 0;
        animationSnapPosDist = 0;
      }
    }
  }

  watch(
    () => props.state,
    (val) => {
      if (val === "hide") exit();
    }
  );

  onMounted(() => {
    if (rdBackground.value && rdPanel.value)
      animate.init(rdBackground.value, rdPanel.value);
  });
</script>

<style lang="scss" scoped>
  .rd-component {
    z-index: 20000;
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    display: flex;
    flex-direction: column;
    .rd-background {
      position: absolute;
      bottom: 0;
      left: 0;
      width: 100vw;
      height: 100vh;
      background: rgba(43, 25, 6, 0.5);
      backdrop-filter: blur(10px);
      opacity: 0;
    }
    .rd-panel {
      touch-action: none;
      position: absolute;
      bottom: 0;
      left: 0;
      width: 100vw;
      background: var(--background-depth-one-color);
      overflow-y: hidden;
      overflow-x: visible;
      border-top-left-radius: 1rem;
      border-top-right-radius: 1rem;
      padding: 0 0 1rem 0;
      box-sizing: border-box;
      display: flex;
      flex-direction: column;
      transform: translateY(100%);
      .rd-panel-header {
        position: relative;
        width: 100%;
        height: 4rem;
        padding: 1rem;
        box-sizing: border-box;
        &::before {
          content: "";
          touch-action: pan-y;
          position: absolute;
          top: 0.375rem;
          left: calc(50% - 0.75rem);
          width: 1.5rem;
          height: 0.25rem;
          border-radius: 0.125rem;
          background: var(--border-color);
        }
      }
      .rd-panel-body {
        position: relative;
        width: 100%;
        padding: 0 1rem;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
      }
    }
    @media only screen and (min-width: 1025px) {
      .rd-panel {
        left: auto;
        bottom: 0;
        right: 0;
        width: 20rem;
        height: 100vh;
        border-top-left-radius: 0;
        border-top-right-radius: 0;
        padding: 0;
        box-sizing: border-box;
        transform: translateX(125%);
        .rd-panel-header {
          height: 6rem;
          padding: 2rem;
          &::before {
            display: none;
          }
        }
        .rd-panel-body {
          padding: 0 2rem;
          height: 100%;
        }
      }
    }
  }
</style>
