import {createApp} from "vue";
import NewStatus from "./new-status/page.vue";
import WithSuspense from "../components/with-suspense.vue";

const app = createApp(WithSuspense, {
  component: NewStatus,
});
app.mount("#app");
