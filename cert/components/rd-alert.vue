<template>
  <div class="rd-component" ref="rdComponent">
    <div
      v-for="alert in alerts"
      :key="alert.id"
      :data-id="alert.id"
      class="rd-alert"
    >
      <div class="rd-alert-icon-container">
        <rd-svg :name="alert.type" color="secondary" />
      </div>
      <div class="rd-alert-details">
        <span class="rd-alert-title rd-headline-5">
          {{ alert.title }}
        </span>
        <span class="rd-alert-message rd-caption-text">
          {{ alert.message }}
        </span>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";
  import { Alert } from "~~/types/alert";

  interface AlertTimeout extends Alert {
    id: number;
    playing: boolean;
    timeout: NodeJS.Timeout;
  }

  const { alert, removeAlert } = useAlert();

  const rdComponent = ref<HTMLDivElement | null>(null);

  const alerts = ref<AlertTimeout[]>([]);
  const alertId = ref<number>(0);

  const animate = {
    show(rdComponent: HTMLElement, id: number, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdAlert: HTMLElement | null = rdComponent.querySelector(
        `.rd-alert[data-id="${id}"]`
      );

      tl.to(rdAlert, {
        y: 0,
        ease: "power2.out",
        duration: 0.5,
      });
    },
    hide(rdComponent: HTMLElement, id: number, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdAlert: HTMLElement | null = rdComponent.querySelector(
        `.rd-alert[data-id="${id}"]`
      );

      tl.to(rdAlert, {
        y: "-4rem",
        ease: "power2.in",
        duration: 0.5,
      });
    },
    remove(rdComponent: HTMLElement, id: number, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdAlert: HTMLElement | null = rdComponent.querySelector(
        `.rd-alert[data-id="${id}"]`
      );

      tl.to(rdAlert, {
        opacity: 0,
        scale: 0.75,
        ease: "power2.in",
        duration: 0.5,
      });
    },
  };

  function alertHandler(data: AlertTimeout): void {
    removeAlert();
    alerts.value.push(data);
    setTimeout(() => {
      const prevIndex: number = alerts.value.length - 2;
      const prevAlert: AlertTimeout = alerts.value[prevIndex];
      if (prevAlert && !prevAlert.playing) {
        alerts.value[prevIndex].playing = true;
        if (rdComponent.value) {
          animate.remove(rdComponent.value, prevAlert.id, () => {
            clearTimeout(prevAlert.timeout);
            alerts.value.splice(prevIndex, 1);
          });
        }
      }
      setTimeout(
        () => {
          if (rdComponent.value) {
            animate.show(rdComponent.value, data.id);
          }
        },
        prevAlert ? 100 : 0
      );
    }, 100);
  }

  watch(
    () => alert.value,
    (val) => {
      if (val) {
        const id: number = alertId.value++;
        alertHandler({
          ...val,
          id,
          playing: false,
          timeout: setTimeout(() => {
            const index: number = alerts.value.findIndex((a) => a.id === id);
            if (
              index > -1 &&
              !alerts.value[index].playing &&
              rdComponent.value
            ) {
              alerts.value[index].playing = true;
              animate.hide(rdComponent.value, id, () => {
                alerts.value.splice(index, 1);
              });
            }
          }, 2000),
        });
      }
    }
  );
</script>

<style lang="scss" scoped>
  .rd-component {
    pointer-events: none;
    z-index: 3000000000;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 5rem;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    .rd-alert {
      position: absolute;
      bottom: 1rem;
      left: calc(50% - 10rem);
      width: 20rem;
      height: 3rem;
      padding: 0.5rem;
      box-sizing: border-box;
      background: var(--primary-color);
      border-radius: 0.75rem;
      display: flex;
      justify-content: flex-start;
      align-items: center;
      transform: translateY(-4rem);
      transform-origin: top center;
      .rd-alert-icon-container {
        position: relative;
        width: 2rem;
        height: 2rem;
        background: rgba(0, 0, 0, 0.05);
        border-radius: 0.5rem;
        padding: 0 0.5rem;
        box-sizing: border-box;
        display: flex;
        justify-content: center;
        align-items: center;
      }
      .rd-alert-details {
        position: relative;
        width: calc(100% - 2rem);
        height: 100%;
        color: var(--font-secondary-color);
        padding-left: 0.5rem;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: flex-start;
        gap: 0.25rem;
        span {
          position: relative;
          line-height: 1;
          color: inherit;
        }
      }
    }
    @media only screen and (max-width: 1024px) {
      .rd-alert {
        left: 1rem;
        width: calc(100vw - 2rem);
      }
    }
  }
</style>
