<template>
  <div class="rd-header" ref="rdHeader">
    <div class="rd-header-images">
      <rd-image-slider :state="state" :images="images" @change="change" />
    </div>
    <h1 v-if="current" class="rd-header-headline rd-title-2">
      <span
        v-for="(word, i) in current.headline"
        :key="i"
        class="rd-word-wrapper"
      >
        <span class="rd-word-container rd-word-container-down">
          <span class="rd-word">{{ word.text }}</span>
        </span>
      </span>
    </h1>
    <div v-if="current" class="rd-header-details">
      <p class="rd-header-description rd-subtitle-text">
        <span
          v-for="(text, i) in current.description.split(' ')"
          :key="i"
          class="rd-text-wrapper"
        >
          <span class="rd-text-container rd-text-container-down">
            <span class="rd-text">{{ text }}</span>
          </span>
        </span>
      </p>
      <div v-if="current.phrase" class="rd-header-phrase">
        <div v-if="current.phrase?.icon" class="rd-header-phrase-icon">
          <rd-svg :name="current.phrase.icon" />
        </div>
        <div v-else class="rd-header-phrase-dash"></div>
        <span class="rd-header-phrase-text rd-headline-5">
          <span
            v-for="(text, i) in current.phrase.text"
            :key="i"
            class="rd-text-wrapper"
          >
            <span class="rd-text-container rd-text-container-down">
              <span class="rd-text">{{ text }}</span>
            </span>
          </span>
        </span>
      </div>
    </div>
    <rd-input-button-attraction
      v-if="current && current.action"
      :label="current.action.text"
      @clicked="handleAction(current.action)"
      :state="buttonState"
      type="secondary"
    />
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
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";
  import { ContentAction, ContentHeaderOne } from "types/content";

  const props = defineProps<{
    data: ContentHeaderOne[];
    state: "uninitialized" | "initialized";
  }>();
  const emits = defineEmits(["change-section"]);

  const rdHeader = ref<HTMLDivElement | null>(null);

  const index = ref<number>(-1);
  const buttonState = ref<"uninitialized" | "initialized">("uninitialized");

  const images = computed<string[]>(() => props.data.map((a) => a.image) || []);
  const current = computed<ContentHeaderOne | undefined>(
    () => props.data[index.value]
  );

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
    exit(rdHeader: HTMLElement, cb: () => void): void {
      const tl = gsap.timeline({
        onComplete() {
          gsap.to(rdDash, {
            x: 0,
            duration: 0,
          });
          gsap.to([...rdWordContainer, ...rdTextContainer], {
            y: "100%",
            duration: 0,
          });
          gsap.to([...rdWord, ...rdText], {
            y: "-100%",
            duration: 0,
          });
          cb();
        },
      });

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

      tl.to(rdWordContainer, {
        y: "-100%",
        duration: 1,
        ease: "expo.inOut",
        stagger: 0.05,
      })
        .to(
          rdWord,
          {
            y: "100%",
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.05,
          },
          "<0"
        )
        .to(
          rdTextContainer,
          {
            y: "-100%",
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.01,
          },
          "<0.25"
        )
        .to(
          rdText,
          {
            y: "100%",
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
            x: "1.5rem",
            width: 0,
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
            opacity: 0,
            duration: 1,
            ease: "power0",
          },
          "<0"
        );
      }
    },
  };

  function change(i: number): void {
    if (rdHeader.value) {
      if (index.value > -1) {
        buttonState.value = "uninitialized";
        animate.exit(rdHeader.value, () => {
          index.value = i;
          setTimeout(() => {
            buttonState.value = "initialized";
            if (rdHeader.value) animate.init(rdHeader.value);
          }, 100);
        });
      } else {
        index.value = i;
        setTimeout(() => {
          buttonState.value = "initialized";
          if (rdHeader.value) animate.init(rdHeader.value);
        }, 1000);
      }
    }
  }
  function handleAction(action: ContentAction): void {
    if (action.kind === "link" && action.target) {
      window.open(action.target);
    } else if (action.kind === "section" && action.target) {
      emits("change-section", action.target);
    }
  }
</script>

<style lang="scss" scoped>
  .rd-header {
    position: relative;
    width: 100vw;
    height: 100vh;
    padding: 2rem;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    align-items: flex-start;
    .rd-header-images {
      position: absolute;
      top: 0.5rem;
      left: 0.5rem;
      width: calc(100% - 1rem);
      height: calc(100% - 1rem);
      border-radius: 2rem;
      overflow: hidden;
    }
    h1.rd-header-headline {
      position: relative;
      width: 50%;
      box-sizing: border-box;
      display: flex;
      flex-wrap: wrap;
      gap: 0 0.75rem;
    }
    .rd-header-details {
      position: relative;
      width: calc(100% - 150px);
      margin: 2rem 0;
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
      position: absolute;
      bottom: 2rem;
      right: 2rem;
      width: 150px;
      height: 150px;
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
    @media only screen and (max-width: 1024px) {
      padding: 1rem;
      h1.rd-header-headline {
        width: 80%;
        padding: 0;
        gap: 0.25rem;
      }
      .rd-header-details {
        padding: 0;
        margin: 1rem 0;
        flex-direction: column;
        align-items: flex-start;
        gap: 1rem;
        .rd-header-phrase {
          width: 100%;
          display: none;
        }
        p.rd-header-description {
          width: 100%;
          gap: 0.1rem;
        }
      }
      .rd-header-scroll {
        display: none;
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
        right: auto;
        left: 2rem;
      }
    }
  }
</style>
