<!-- component to make loading fonts easier - https://github.com/Exponential-Workload/create-ts/blob/main/templateFiles/svelte/src/lib/NexusFonts/Fonts.svelte -->
<script lang="ts">
    import { AllowLocal, FontDisplay, type Font, type _Weight } from './Fonts';
    import FontsPreloads from './FontsPreloads.svelte';
    /** It's recommended to load as many fonts in 1 Font instance at once, as we make them all into 1 request per Font element | The font element MUST be in a prerendered or SSRed document */
    export let fonts: Font[] | Font;
    /** How to display the font */
    export let display: FontDisplay = FontDisplay.Swap;
    /** Allow usage of local fonts; can break shit */
    export let local: AllowLocal = AllowLocal.Never;
    if (!fonts) fonts = [];
    if (typeof fonts !== 'object' || !(fonts as any).forEach)
      fonts = [fonts as Font];
    const fontsAsStrArray = (fonts as Font[]).map((v) => {
      if (typeof v === 'string') return v;
      let {
        family,
        weight: _weight,
        italic,
      } = {
        italic: false,
        weight: 500,
        ...v,
      };
      if (!family) throw new Error('Font as array does not have family');
      let weight: _Weight[];
      if (typeof _weight !== 'object') weight = [_weight];
      else weight = _weight;
      const defaultWeight = weight.length > 1 || (weight[0] && weight[0] !== 500);
      const defualtItal =
        !italic ||
        (typeof italic === 'object' && italic.length === 1 && !italic[0]);
      const tags = [
        defaultWeight ? '' : 'wght',
        defualtItal ? '' : 'ital',
      ].filter((v) => v.length > 0);
      if (tags.length > 0) {
        let thisFamily = `${family}:${tags.join(',')}@`;
        const pairs: [_Weight, boolean][] = [];
        weight.forEach((v, idx) => {
          pairs[idx] = [
            v,
            typeof italic === 'boolean'
              ? italic
              : italic[idx] ?? italic[0] ?? false,
          ];
        });
        if (typeof italic === 'boolean' && weight.length === 0) {
          pairs[0] = [500, italic];
        } else if (typeof italic !== 'boolean') {
          italic.forEach((v, idx) => {
            pairs[idx] = pairs[idx] ?? [weight[idx] ?? weight[0] ?? 500, v];
          });
        }
        // console.log(tags, pairs);
  
        pairs.forEach(([w, i], dx) => {
          thisFamily += `${tags.includes('wght') ? w : ''}${
            tags.length >= 2 ? ',' : ''
          }${tags.includes('ital') ? (i ? '1' : '0') : ''}${
            dx === pairs.length - 1 ? '' : ';'
          }`;
        });
        return thisFamily;
      } else return family;
    });
    const href = `https://fonts.nexuspipe.com/css2?family=${fontsAsStrArray.join(
      '|'
    )}&display=${display}${local !== AllowLocal.Auto ? `&local=${local}` : ''}`;
  
    /** Either {@link FontsPreloads} should be included once in a svelte:head, or this should be true once. */
    export let shouldPreload = false;
  </script>
  
  {#if shouldPreload}
    <FontsPreloads />
  {/if}
  <link href={encodeURI(href)} rel="stylesheet" />