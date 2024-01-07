export default defineNuxtRouteMiddleware(async (to, from) => {
  const { getContent } = useContent();

  const language = to.params.language.toString();
  const page = to.params.page.toString();

  if (process.client && !(await getContent(language, page))) {
    return "/admin";
  }
  return true;
});
