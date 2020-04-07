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
      <tr class="template" v-for="(template, index) in templates" :key="index">
        <td class="name">{{ template.name }}</td>
        <td class="description">{{ template.description }}</td>
        <td><input type="button" value="Edit Template" @click="open_template(index)" /></td>
      </tr>
    </table>
    <p><input type="button" value="Create New Template" @click="create_new_template" /></p>
    <p><input type="button" value="Refresh" @click="load_templates" /></p>
    <TemplateEditor @close="editing_template = false" 
                    :template_id="current_template_id" 
                    :template="current_template"
                    v-if="editing_template" 
                    @submitted="on_template_finish" />
  </div>
</template>

<script lang="ts">
import axios from "axios";
import Vue from "vue";
import Component from "vue-class-component";
import { Template } from "@/types";

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
    editing_template = false;
    current_template: Template | null = null;
    current_template_id = -1;

    async load_templates(): Promise<void> {
      if (!this.loading) {
        this.templates = [];

        this.loading = true;
        let { data } = await axios.post("/api/list_templates", {});
        this.loading = false;
        
        data.forEach((item: any[]) => {
          this.templates.push({
            id: item[0],
            name: item[1],
            description: item[2],
          });
        });
      }
    }

    create_new_template() {
        this.current_template = null;
        this.current_template_id = -1;
        this.editing_template = true;
    }

    on_template_finish() {
        location.reload();
    }

    async open_template(i: number): Promise<void> {
        if (!this.loading) {
            this.current_template_id = this.templates[i].id;
            this.loading = true;
            let { data } = await axios.post("/api/get_template", { tid: this.current_template_id });
            this.loading = false;
            this.current_template = data;
            this.editing_template = true;
        }
    } 
}
</script>

<style scoped lang="scss">
table.templates {
  border-collapse: collapse;
  border: 1px solid black;

  .template {
    td {
      border: 1px solid black;
      margin: auto;
      vertical-align: middle;
    }
 
    .name {
      text-align: center;
    }

    .description {
      width: 60%;
      overflow: scroll;
      margin-left: 2px;
    }
  }
}
</style>
