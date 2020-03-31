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
  <tr>
    <td>Roll </td>
    <td><input type="number" v-model="dice_number" placeholder="1" min="1" /></td>
    <td>d{{ dice_type }}</td>
    <td><input type="button" v-on:click="roll_die" value="Roll!"/></td>
    <td>
      <span v-if="results.length != 0">: </span>
      <span v-for="result in results" :key="result.index">
        <span>{{ result.val }}</span>
        <span v-if="result.index != results.length - 1"> + </span> 
      </span>
      <span v-if="results.length > 1"> = {{ get_sum() }}</span>
    </td>
  </tr>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import { Prop } from "vue-property-decorator";
import { in_range } from "../random";

interface DiceRoll {
  val: number;
  index: number;
};

@Component({})
export default class DiceRoller extends Vue {
  @Prop() dice_type!: number;
  dice_number = 1;
  results: DiceRoll[] = [];

  roll_die() {
    this.results = [];
    for (let i = 0; i < this.dice_number; i++) {
      this.results.push({ val: in_range(1, this.dice_type), index: i });
    }
  }

  get_sum(): number {
    let sum = 0;
    this.results.forEach((i: any) => sum += i.val);
    return sum;
  }
};
</script>

<style scoped lang="scss">

</style>
