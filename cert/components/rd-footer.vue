<template>
  <div class="rd-footer" ref="rdFooter">
    <h1 class="rd-footer-headline rd-title-1">
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
    <div class="rd-footer-details">
      <p class="rd-footer-description rd-subtitle-text">
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
    <div v-if="data.link.length" class="rd-footer-links">
      <a
        v-for="(link, i) in data.link"
        :key="i"
        class="rd-footer-link"
        :href="link.url"
        @mouseenter="mouseEnter"
        @mouseleave="mouseLeave"
      >
        <div class="rd-footer-link-container">
          <span class="rd-footer-link-name rd-headline-3">
            <span class="rd-word-wrapper">
              <span class="rd-word-container rd-word-container-down">
                <span class="rd-word">{{ link.name }}</span>
              </span>
            </span>
          </span>
          <div class="rd-footer-link-icon-container">
            <span class="rd-word-wrapper">
              <span class="rd-word-container rd-word-container-down">
                <span class="rd-word"
                  ><rd-svg
                    name="arrow-top-right"
                    class="rd-footer-link-icon"
                    color="secondary"
                /></span>
              </span>
            </span>
          </div>
        </div>
        <div class="rd-footer-link-overlay-wrapper">
          <div class="rd-footer-link-overlay-container">
            <div class="rd-footer-link-overlay-marquee">
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
              <span class="rd-footer-link-overlay-name rd-headline-3">{{
                link.name
              }}</span>
              <div class="rd-footer-link-overlay-icon-container">
                <rd-svg
                  name="arrow-top-right"
                  class="rd-footer-link-overlay-icon"
                />
              </div>
            </div>
          </div>
        </div>
      </a>
    </div>
    <div class="rd-footer-section">
      <div v-if="data.page.length" class="rd-footer-sitemap-container">
        <span class="rd-footer-sitemap-label rd-headline-5">
          <span class="rd-text-wrapper">
            <span class="rd-text-container rd-text-container-down">
              <span class="rd-text">Sitemap</span>
            </span>
          </span>
        </span>
        <a
          v-for="page in data.page"
          :key="page.path"
          :href="page.path"
          class="rd-footer-sitemap"
          @click.prevent="handlePage(page.path)"
        >
          <span class="rd-footer-sitemap-name rd-headline-4">
            <span class="rd-text-wrapper">
              <span class="rd-text-container rd-text-container-down">
                <span class="rd-text">{{ page.name }}</span>
              </span>
            </span>
          </span>
        </a>
      </div>
      <div v-if="data.contact" class="rd-footer-contact-container">
        <span class="rd-footer-contact-label rd-headline-5">
          <span class="rd-text-wrapper">
            <span class="rd-text-container rd-text-container-down">
              <span class="rd-text">Get on touch</span>
            </span>
          </span>
        </span>
        <span
          v-if="data.contact.address"
          class="rd-footer-contact-address rd-headline-4"
        >
          <span
            v-for="(text, i) in data.contact.address.split(' ')"
            :key="i"
            class="rd-text-wrapper"
          >
            <span class="rd-text-container rd-text-container-down">
              <span class="rd-text">{{ text }}</span>
            </span>
          </span>
        </span>
        <rd-input-button-attraction
          v-if="data.contact.email"
          :label="data.contact.email"
          :state="state"
          style="margin-top: 2rem"
        />
      </div>
    </div>
    <div class="rd-footer-end">
      <span class="rd-footer-end-message rd-caption-text">
        <span
          v-for="(text, i) in `${config?.name || env.base} created by`.split(
            ' '
          )"
          :key="i"
          class="rd-text-wrapper"
        >
          <span class="rd-text-container rd-text-container-down">
            <span class="rd-text">{{ text }}</span>
          </span>
        </span>
        <span class="rd-text-wrapper rd-footer-end-message-link">
          <span class="rd-text-container rd-text-container-down">
            <span class="rd-text">redian.id</span>
          </span>
        </span>
      </span>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";
  import { ContentAction, ContentFooter } from "types/content";

  const props = defineProps<{
    data: ContentFooter;
    state: "uninitialized" | "initialized";
  }>();
  const emits = defineEmits(["change-section", "change-page"]);
  const { config } = useConfig();
  const { public: env } = useRuntimeConfig();

  const rdFooter = ref<HTMLDivElement | null>(null);

  const animate = {
    init(rdFooter: HTMLElement): void {
      const tl = gsap.timeline();

      const rdWordContainer: HTMLElement[] = gsap.utils.toArray(
        rdFooter.querySelectorAll(".rd-word-container")
      );
      const rdWord: HTMLElement[] = gsap.utils.toArray(
        rdFooter.querySelectorAll(".rd-word")
      );
      const rdTextContainer: HTMLElement[] = gsap.utils.toArray(
        rdFooter.querySelectorAll(".rd-text-container")
      );
      const rdText: HTMLElement[] = gsap.utils.toArray(
        rdFooter.querySelectorAll(".rd-text")
      );
      const rdLink: HTMLElement[] = gsap.utils.toArray(
        rdFooter.querySelectorAll("a.rd-footer-link")
      );

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
          "<0"
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
        )
        .to(
          rdLink,
          {
            x: 0,
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.1,
          },
          "<0"
        );
    },
  };

  function handleAction(action: ContentAction): void {
    if (action.kind === "link" && action.target) {
      window.open(action.target);
    } else if (action.kind === "section" && action.target) {
      emits("change-section", action.target);
    }
  }
  function handlePage(path: string): void {
    emits("change-page", path);
  }

  function mouseEnter(e: MouseEvent): void {
    if (e.target instanceof HTMLElement) {
      const { height, top } = e.target.getBoundingClientRect();
      const bound = top + height / 2;
      let from: "up" | "down" = "up";
      if (e.y > bound) {
        from = "down";
      }

      const tl = gsap.timeline();
      const target = e.target.children[1];
      gsap.killTweensOf(target);
      gsap.killTweensOf(target.children[0]);

      if (!e.target.dataset["from"]) {
        tl.fromTo(
          target,
          {
            y: from === "up" ? "-110%" : "110%",
          },
          {
            y: 0,
            duration: 0.6,
            ease: "expo",
          }
        ).fromTo(
          target.children[0],
          {
            y: from === "up" ? "110%" : "-110%",
          },
          {
            y: 0,
            duration: 0.6,
            ease: "expo",
          },
          "<0"
        );
      } else {
        tl.to([target, target.children[0]], {
          y: 0,
          duration: 0.6,
          ease: "expo",
        });
      }
    }
  }
  function mouseLeave(e: MouseEvent): void {
    if (e.target instanceof HTMLElement) {
      const { height, top } = e.target.getBoundingClientRect();
      const bound = top + height / 2;
      let to: "up" | "down" = "up";
      if (e.y > bound) {
        to = "down";
      }

      e.target.setAttribute("data-from", to);

      const tl = gsap.timeline();
      const target = e.target.children[1];
      gsap.killTweensOf(target);
      gsap.killTweensOf(target.children[0]);

      tl.to(target, {
        y: to === "up" ? "-110%" : "110%",
        duration: 0.6,
        ease: "expo",
      }).to(
        target.children[0],
        {
          y: to === "up" ? "110%" : "-110%",
          duration: 0.6,
          ease: "expo",
        },
        "<0"
      );
    }
  }

  watch(
    () => props.state,
    (val) => {
      if (val === "initialized" && rdFooter.value) {
        animate.init(rdFooter.value);
      }
    }
  );

  onMounted(() => {
    if (props.state === "initialized" && rdFooter.value) {
      animate.init(rdFooter.value);
    }
  });
</script>

<style lang="scss" scoped>
  .rd-footer {
    position: relative;
    width: 100%;
    padding: 6rem 6rem 0 6rem;
    background: var(--font-primary-color);
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    h1.rd-footer-headline {
      position: relative;
      width: 100%;
      padding: 0 3rem;
      box-sizing: border-box;
      display: flex;
      flex-wrap: wrap;
      gap: 0 1.5rem;
      color: var(--font-secondary-color);
    }
    .rd-footer-details {
      position: relative;
      width: 100%;
      padding: 0 3rem;
      margin-top: 3rem;
      box-sizing: border-box;
      display: flex;
      gap: 3rem;
      p.rd-footer-description {
        position: relative;
        width: 40%;
        display: flex;
        flex-wrap: wrap;
        gap: 0.2rem;
        color: var(--font-secondary-color);
      }
    }
    .rd-footer-links {
      position: relative;
      left: -6rem;
      width: calc(100% + 12rem);
      margin-top: 3rem;
      display: flex;
      flex-direction: column;
      a.rd-footer-link {
        position: relative;
        width: 100%;
        height: 6rem;
        border-bottom: 1px solid var(--font-secondary-color);
        box-sizing: border-box;
        color: var(--font-secondary-color);
        display: flex;
        overflow: hidden;
        transform: translateX(-110%);
        * {
          pointer-events: none;
        }
        .rd-footer-link-container {
          position: absolute;
          width: 100%;
          height: 100%;
          padding: 0 9rem;
          box-sizing: border-box;
          display: flex;
          justify-content: space-between;
          align-items: center;
          .rd-footer-link-icon-container {
            position: relative;
            width: 6rem;
            height: 6rem;
            padding: 2rem;
            box-sizing: border-box;
          }
        }
        .rd-footer-link-overlay-wrapper {
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          background: var(--primary-color);
          overflow: hidden;
          transform: translateY(-110%);
          .rd-footer-link-overlay-container {
            position: relative;
            width: 100%;
            height: 100%;
            overflow: hidden;
            transform: translateY(110%);
            .rd-footer-link-overlay-marquee {
              position: relative;
              width: 200vw;
              height: 100%;
              box-sizing: border-box;
              display: flex;
              flex-shrink: 0;
              justify-content: space-around;
              align-items: center;
              animation: rd-marquee 20s linear infinite;
              span.rd-footer-link-overlay-name {
                pointer-events: none;
                position: relative;
                color: var(--font-primary-color);
                white-space: nowrap;
                text-transform: uppercase;
                display: flex;
                flex-shrink: 0;
                justify-content: center;
                align-items: center;
              }
              .rd-footer-link-overlay-icon-container {
                position: relative;
                width: 6rem;
                height: 6rem;
                padding: 2rem;
                box-sizing: border-box;
              }
            }
          }
        }
        &:first-child {
          border-top: 1px solid var(--font-secondary-color);
        }
        &:focus {
          outline-color: var(--primary-color);
        }
      }
    }
    .rd-footer-section {
      position: relative;
      width: 100%;
      padding: 0 3rem;
      margin-top: 3rem;
      box-sizing: border-box;
      display: flex;
      .rd-footer-sitemap-container,
      .rd-footer-contact-container {
        position: relative;
        width: 50%;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        span.rd-footer-sitemap-label,
        span.rd-footer-contact-label {
          position: relative;
          color: var(--font-secondary-color);
          margin-bottom: 1rem;
          display: flex;
          align-items: center;
          opacity: 0.5;
        }
        span.rd-footer-contact-address {
          position: relative;
          width: 100%;
          padding-right: 15vw;
          box-sizing: border-box;
          color: var(--font-secondary-color);
          display: flex;
          flex-wrap: wrap;
          gap: 0.2rem;
        }
        a.rd-footer-sitemap {
          position: relative;
          width: 100%;
          height: 2rem;
          display: flex;
          justify-content: space-between;
          align-items: center;
          transition: 0.2s opacity;
          text-decoration: none;
          * {
            pointer-events: none;
          }
          span.rd-footer-sitemap-name {
            position: relative;
            color: var(--font-secondary-color);
          }
          &:hover {
            opacity: 0.5;
          }
        }
      }
    }
    .rd-footer-end {
      position: relative;
      left: -6rem;
      width: calc(100% + 12rem);
      height: 3rem;
      margin-top: 3rem;
      display: flex;
      justify-content: center;
      align-items: center;
      span.rd-footer-end-message {
        position: relative;
        color: var(--font-secondary-color);
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.1rem;
        span.rd-text-wrapper {
          opacity: 0.5;
          * {
            pointer-events: none;
          }
        }
        span.rd-footer-end-message-link {
          cursor: pointer;
          opacity: 0.5;
          transition: 0.2s opacity, 0.2s color;
          &:hover {
            opacity: 1;
            color: var(--primary-color);
          }
        }
      }
      &::before {
        content: "";
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 1px;
        background: var(--font-secondary-color);
        opacity: 0.25;
      }
    }
    @media only screen and (max-width: 1024px) {
      padding: 2rem 1rem 0 1rem;
      h1.rd-footer-headline {
        padding: 0;
        gap: 0.25rem;
      }
      .rd-footer-details {
        padding: 0;
        margin-top: 1.5rem;
        flex-direction: column;
        align-items: flex-start;
        gap: 1.5rem;
        .rd-footer-phrase {
          width: 100%;
        }
        p.rd-footer-description {
          width: 100%;
          gap: 0.1rem;
        }
      }
      .rd-footer-links {
        left: -1rem;
        width: calc(100% + 2rem);
        margin-top: 1.5rem;
        a.rd-footer-link {
          height: 4rem;
          .rd-footer-link-container {
            padding: 1rem 0 1rem 1rem;
            .rd-footer-link-icon-container {
              width: 4rem;
              height: 4rem;
              padding: 1.5rem;
            }
          }
          .rd-footer-link-overlay-wrapper {
            .rd-footer-link-overlay-container {
              .rd-footer-link-overlay-marquee {
                .rd-footer-link-overlay-icon-container {
                  width: 4rem;
                  height: 4rem;
                  padding: 1.5rem;
                }
              }
            }
          }
        }
      }
      .rd-footer-section {
        padding: 0;
        margin-top: 1rem;
        flex-wrap: wrap;
        gap: 2rem;
        .rd-footer-sitemap-container,
        .rd-footer-contact-container {
          width: 100%;
          span.rd-footer-contact-address {
            padding-right: 15vw;
            gap: 0.2rem;
          }
        }
      }
      .rd-footer-end {
        left: -1rem;
        width: calc(100% + 2rem);
        margin-top: 2rem;
      }
    }
  }
  html[dir="rtl"] {
    .rd-footer {
      .rd-footer-links {
        left: auto;
        right: -6rem;
        a.rd-footer-link {
          .rd-footer-link-overlay-wrapper {
            .rd-footer-link-overlay-container {
              .rd-footer-link-overlay-marquee {
                animation: rd-marquee-rtl 20s linear infinite;
              }
            }
          }
        }
      }
      .rd-footer-end {
        left: auto;
        right: -6rem;
      }
      @media only screen and (max-width: 1024px) {
        .rd-footer-links {
          left: auto;
          right: -1rem;
          a.rd-footer-link {
            .rd-footer-link-container {
              padding: 1rem 1rem 1rem 0;
            }
          }
        }
        .rd-footer-section {
          .rd-footer-sitemap-container,
          .rd-footer-contact-container {
            span.rd-footer-contact-address {
              padding-right: 0;
              padding-left: 15vw;
            }
          }
        }
        .rd-footer-end {
          left: auto;
          right: -1rem;
        }
      }
    }
  }
</style>
