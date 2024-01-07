<template>
  <div
    class="rd-header"
    ref="rdHeader"
    :class="data.image?.length ? 'rd-header-image' : ''"
  >
    <h1 class="rd-header-headline rd-title-1">
      <span
        v-for="(word, i) in data.headline"
        :key="i"
        :style="
          word.color !== 'default' ? `color: var(--${word.color}-color);` : ''
        "
        class="rd-word-wrapper"
      >
        <span class="rd-word-container rd-word-container-down">
          <span class="rd-word">{{ word.text }}</span>
        </span>
      </span>
    </h1>
    <div class="rd-header-details">
      <div v-if="data.phrase" class="rd-header-phrase">
        <div v-if="data.phrase.icon" class="rd-header-phrase-icon">
          <rd-svg :name="data.phrase.icon" />
        </div>
        <div v-else class="rd-header-phrase-dash"></div>
        <span class="rd-header-phrase-text rd-headline-5">
          <span
            v-for="(text, i) in data.phrase.text"
            :key="i"
            class="rd-text-wrapper"
          >
            <span class="rd-text-container rd-text-container-down">
              <span class="rd-text">{{ text }}</span>
            </span>
          </span>
        </span>
      </div>
      <p class="rd-header-description rd-subtitle-text">
        <span
          v-for="(text, i) in data.description.split(' ')"
          :key="i"
          class="rd-text-wrapper"
        >
          <span class="rd-text-container rd-text-container-down">
            <span class="rd-text">{{ text }}</span>
          </span>
        </span>
      </p>
      <rd-input-button-attraction
        v-if="data.action"
        :label="data.action.text"
        :state="state"
        @clicked="handleAction(data.action)"
      />
    </div>
    <div class="rd-header-scroll">
      <rd-svg
        class="rd-header-scroll-roller"
        name="scrolldown"
        color="primary"
      />
      <rd-svg
        class="rd-header-scroll-icon"
        name="arrow-alt-down"
        color="primary"
      />
    </div>
    <div v-if="data.image?.length" class="rd-header-images">
      <rd-image-slider :state="state" :images="data.image" />
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";
  import { ContentAction, ContentHeaderTwo } from "types/content";

  const props = defineProps<{
    data: ContentHeaderTwo;
    state: "uninitialized" | "initialized";
  }>();
  const emits = defineEmits(["change-section"]);

  const rdHeader = ref<HTMLDivElement | null>(null);

  const animate = {
    init(rdHeader: HTMLElement): void {
      const tl = gsap.timeline();

      const rdWordContainer: HTMLElement[] = gsap.utils.toArray(
        rdHeader.querySelectorAll(".rd-word-container")
      );
      const rdWord: HTMLElement[] = gsap.utils.toArray(
        rdHeader.querySelectorAll(".rd-word")
      );
      const rdTextContainer: HTMLElement[] = gsap.utils.toArray(
        rdHeader.querySelectorAll(".rd-text-container")
      );
      const rdText: HTMLElement[] = gsap.utils.toArray(
        rdHeader.querySelectorAll(".rd-text")
      );
      const rdDash: HTMLElement | null = rdHeader.querySelector(
        ".rd-header-phrase-dash"
      );
      const rdIcon: HTMLElement | null = rdHeader.querySelector(
        ".rd-header-phrase-icon"
      );
      const rdScroll: HTMLElement | null =
        rdHeader.querySelector(".rd-header-scroll");

      tl.to(rdWordContainer, {
        y: 0,
        duration: 1,
        ease: "expo.inOut",
        stagger: 0.05,
      })
        .to(
          rdWord,
          {
            y: 0,
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.05,
          },
          "<0"
        )
        .to(
          rdTextContainer,
          {
            y: 0,
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.01,
          },
          "<0.25"
        )
        .to(
          rdText,
          {
            y: 0,
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.01,
          },
          "<0"
        );

      if (rdDash) {
        tl.to(
          rdDash,
          {
            width: "1.5rem",
            duration: 1,
            ease: "expo.inOut",
          },
          "<0"
        );
      }
      if (rdIcon) {
        tl.to(
          rdIcon,
          {
            opacity: 1,
            duration: 1,
            ease: "power0",
          },
          "<0"
        );
      }
      if (rdScroll) {
        tl.to(
          rdScroll,
          {
            opacity: 1,
            duration: 0.25,
            ease: "power0",
          },
          ">0.25"
        );
      }
    },
  };

  function handleAction(action: ContentAction): void {
    if (action.kind === "link" && action.target) {
      window.location.href = action.target;
    } else if (action.kind === "section" && action.target) {
      emits("change-section", action.target);
    }
  }

  watch(
    () => props.state,
    (val) => {
      if (val === "initialized" && rdHeader.value) {
        animate.init(rdHeader.value);
      }
    }
  );

  onMounted(() => {
    if (props.state === "initialized" && rdHeader.value) {
      animate.init(rdHeader.value);
    }
  });
</script>

<style lang="scss" scoped>
  .rd-header {
    position: relative;
    width: 100%;
    padding: 12rem 6rem 0 6rem;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    h1.rd-header-headline {
      position: relative;
      width: 100%;
      padding: 0 3rem;
      box-sizing: border-box;
      display: flex;
      flex-wrap: wrap;
      gap: 0 1.5rem;
    }
    .rd-header-details {
      position: relative;
      width: 100%;
      padding: 0 3rem;
      margin-top: 3rem;
      box-sizing: border-box;
      display: flex;
      gap: 3rem;
      .rd-header-phrase {
        position: relative;
        height: 2rem;
        display: flex;
        align-items: center;

        .rd-header-phrase-dash {
          position: absolute;
          width: 0;
          height: 2px;
          background: var(--font-primary-color);
        }
        .rd-header-phrase-icon {
          position: absolute;
          width: 1.5rem;
          height: 1.5rem;
          opacity: 0;
        }
        span.rd-header-phrase-text {
          position: relative;
          height: 100%;
          margin-left: 2rem;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: flex-start;
        }
      }
      p.rd-header-description {
        position: relative;
        width: 40%;
        display: flex;
        flex-wrap: wrap;
        gap: 0.2rem;
      }
    }
    .rd-header-scroll {
      z-index: 2;
      position: relative;
      width: 150px;
      height: 150px;
      margin: 3rem 0 0 3rem;
      border-radius: 50%;
      background: var(--font-primary-color);
      display: flex;
      justify-content: center;
      align-items: center;
      opacity: 0;
      * {
        pointer-events: none;
      }
      .rd-header-scroll-roller {
        position: absolute;
        left: 10px;
        top: 10px;
        width: 130px;
        height: 130px;
        animation: rd-rotate 10s linear infinite;
      }
      .rd-header-scroll-icon {
        position: absolute;
        top: 50px;
        left: 50px;
        width: 50px;
        height: 50px;
      }
    }
    .rd-header-images {
      z-index: 1;
      position: relative;
      width: 100%;
      aspect-ratio: 16 / 9;
      border-radius: 2rem;
      overflow: hidden;
      * {
        border-radius: 2rem;
      }
    }
    &.rd-header-image {
      .rd-header-scroll {
        margin: 1.5rem 3rem -75px 0;
        align-self: flex-end;
      }
    }
    @media only screen and (max-width: 1024px) {
      padding: 8rem 1rem 0 1rem;
      h1.rd-header-headline {
        padding: 0;
        gap: 0.25rem;
      }
      .rd-header-details {
        padding: 0;
        margin-top: 1.5rem;
        flex-direction: column;
        align-items: flex-start;
        gap: 1.5rem;
        .rd-header-phrase {
          width: 100%;
        }
        p.rd-header-description {
          width: 100%;
          gap: 0.1rem;
        }
      }
      .rd-header-scroll {
        position: absolute;
        bottom: calc(3rem - 100px);
        right: 1rem;
        width: 100px;
        height: 100px;
        margin: 0;
        .rd-header-scroll-roller {
          width: 80px;
          height: 80px;
        }
        .rd-header-scroll-icon {
          top: 37.5px;
          left: 37.5px;
          width: 25px;
          height: 25px;
        }
      }
      &.rd-header-image {
        .rd-header-scroll {
          position: relative;
          bottom: auto;
          right: auto;
          margin: 0.75rem 1.5rem -50px 0;
        }
      }
    }
  }
  html[dir="rtl"] {
    .rd-header {
      .rd-header-details {
        .rd-header-phrase {
          span.rd-header-phrase-text {
            margin-left: 0;
            margin-right: 2rem;
          }
        }
      }
      .rd-header-scroll {
        margin: 3rem 0 0 3rem;
      }
      &.rd-header-image {
        .rd-header-scroll {
          margin: 1.5rem 0 -75px 3rem;
        }
      }
      @media only screen and (max-width: 1024px) {
        .rd-header-scroll {
          right: auto;
          left: 1.5rem;
        }
        &.rd-header-image {
          .rd-header-scroll {
            margin: 0.75rem 0 -50px 0;
          }
        }
      }
    }
  }
</style>
