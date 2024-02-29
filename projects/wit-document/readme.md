Title
=====

用 scss 实现以下效果

type-icon 的字母要加一个圈

span 的背景颜色根据 td.class 来决定, type-resource 是红圈, type-class 是蓝圈

a 的字体颜色根据 td.class 来决定, type-resource 是加深红字, type-class 是加深蓝字

例子为:

```html
<td class="type-resource"><span class="type-icon">R</span><a href="fields">fields</a></td>
<td class="type-class"><span class="type-icon">R</span><a href="classes">classes</a></td>
```