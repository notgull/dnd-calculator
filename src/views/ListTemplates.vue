<--
  Copyright 2020 not_a_seagull

  Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated 
  documentation files (the "Software"), to deal in the Software without restriction, including without limitation
  the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, 
  and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in all copies or substantial portions 
  of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED 
  TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL 
  THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF 
  CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER 
  DEALINGS IN THE SOFTWARE.
 -->

<template>
  <div>
    <p v-show="loading">Loading templates...</p>
    <table class="templates">
      <tr class="template" v-for="template in templates">
        <td>{{ template.name }}</td>
        <td class="description">{{ template.description }}</td>
        <td></td>
      </tr>
    </table>
    <p><input type="button" value="Create New Template" @click="create_new_template" /></p>
    <p><input type="button" value="Refresh" @click="load_templates" /></p>
    <TemplateEditor :is_new_template="false" :init_template_id="-1" v-if="creating_new_template" :init_template_data="null" />
  </div>
</template>

<script lang="ts">
import axios from "axios";
import Vue from "vue";
import Component from "vue-class-component";

import TemplateEditor from "@/components/TemplateEditor.vue";

interface TemplateDescription {
  id: number,
  name: string,
  description: string,
};

@Component({
    components: { TemplateEditor },
    async created(this: ListTemplates): Promise<void> {
        return this.load_templates();
    }
})
export default class ListTemplates extends Vue {
    loading: boolean = false;
    templates: TemplateDescription[] = [];
    creating_new_template = false;

    async load_templates(): Promise<void> {
      if (!this.loading) {
        this.loading = true;
        let res = await axios.post("/api/list_templates", {});
        this.loading = false;
        console.log(res);
      }
    }

    create_new_template() {
        this.creating_new_template = true;
    }
}
</script>

<style scoped lang="scss">
table.templates {
  border: 1;

  .template {
    .description {
      width: 60%;
      overflow: scroll;
    }
  }
}
</style>
