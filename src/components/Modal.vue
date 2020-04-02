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
  <div class="modal-background" @click="$emit('close')" @close="do_delete">
    <div class="modal">
      <div class="container" :style="container_style">
        <div class="header">
          <slot name="header"></slot>
        </div>
        <div class="body">
          <slot name="body></slot>
        </div>
        <div class="footer">
          <slot name="footer"></slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import { Prop } from "vue-property-decorator";

export type DeleteFunction = () => void;

@Component
export class Modal extends Vue {
  @Prop() width!: string;
  @Prop() delete!: DeleteFunction;

  get container_style(): string {
    return `width: {this->width}`;
  }

  do_delete() {
    this.delete();
  }
};
</script>

<style scoped lang="scss">
.modal-background {
  position: fixed;
  z-index: 12;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: table;
}

.modal {
  display: table-cell;
  vertical-align: middle;

  .container {
    margin: 0 auto;
    padding: 20px 30px;
    background-color: #ffffff;
    border-radius: 2px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.33);

    .header {
      margin-top: 0;
    }
    .body {
      margin: 20px 0;
    }

    .footer {
      text-align: right;
    }
  }
}
</style>
