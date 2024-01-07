<template>
  <div
    class="rd-component-svg"
    ref="rdComponent"
    :class="color ? `rd-color-${color}` : ''"
  ></div>
</template>

<script lang="ts" setup>
  const config = useRuntimeConfig();
  const props = defineProps<{
    name: string;
    color?: "success" | "error" | "primary" | "secondary";
  }>();

  const rdComponent = ref<HTMLDivElement | null>(null);

  const rdElementText = ref<string>("");

  onBeforeMount(async () => {
    await loadElementText();
  });

  async function loadElementText(): Promise<boolean> {
    try {
      const data: Response = await fetch(
        `${config.public.base}/icons/${props.name}.svg`
      );
      rdElementText.value = await data.text();
      return true;
    } catch {
      return false;
    }
  }

  watch(
    () => rdElementText.value,
    (val) => {
      if (rdComponent.value) rdComponent.value.innerHTML = val;
    }
  );

  watch(
    () => props.name,
    () => {
      loadElementText();
    }
  );
</script>

<style lang="scss">
  .rd-component-svg {
    position: relative;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: 0.25s transform;
    svg {
      position: relative;
      width: 100%;
      height: 100%;
      &.rd-svg-filled {
        .rd-svg-filled-main {
          fill: var(--font-primary-color);
        }
        .rd-svg-filled-secondary {
          fill: var(--font-secondary-color);
        }
      }
      &.rd-svg-outlined {
        .rd-svg-outlined-main {
          stroke: var(--font-primary-color);
        }
        .rd-svg-outlined-secondary {
          stroke: var(--font-secondary-color);
        }
      }
    }
    &.rd-color-secondary {
      svg {
        &.rd-svg-filled {
          .rd-svg-filled-main {
            fill: var(--font-secondary-color);
          }
          .rd-svg-filled-secondary {
            fill: var(--font-primary-color);
          }
        }
        &.rd-svg-outlined {
          .rd-svg-outlined-main {
            stroke: var(--font-secondary-color);
          }
          .rd-svg-outlined-secondary {
            stroke: var(--font-primary-color);
          }
        }
      }
    }
    &.rd-color-primary {
      svg {
        &.rd-svg-filled {
          .rd-svg-filled-main {
            fill: var(--primary-color);
          }
          .rd-svg-filled-secondary {
            fill: var(--secondary-color);
          }
        }
        &.rd-svg-outlined {
          .rd-svg-outlined-main {
            stroke: var(--primary-color);
          }
          .rd-svg-outlined-secondary {
            stroke: var(--secondary-color);
          }
        }
      }
    }
    &.rd-color-error {
      svg {
        &.rd-svg-filled {
          .rd-svg-filled-main {
            fill: var(--error-color);
          }
          .rd-svg-filled-secondary {
            fill: var(--secondary-color);
          }
        }
        &.rd-svg-outlined {
          .rd-svg-outlined-main {
            stroke: var(--error-color);
          }
          .rd-svg-outlined-secondary {
            stroke: var(--secondary-color);
          }
        }
      }
    }
    &.rd-color-success {
      svg {
        &.rd-svg-filled {
          .rd-svg-filled-main {
            fill: var(--success-color);
          }
          .rd-svg-filled-secondary {
            fill: var(--secondary-color);
          }
        }
        &.rd-svg-outlined {
          .rd-svg-outlined-main {
            stroke: var(--success-color);
          }
          .rd-svg-outlined-secondary {
            stroke: var(--secondary-color);
          }
        }
      }
    }
  }
</style>
