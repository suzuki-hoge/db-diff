import type { Meta, StoryObj } from '@storybook/react'

import { ColorTag } from './ColorTag'

const meta = {
  title: 'Atoms/ColorTag',
  component: ColorTag,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof ColorTag>

export default meta
type Story = StoryObj<typeof meta>

export const Red: Story = {
  args: {
    color: 'red',
  },
}

export const Green: Story = {
  args: {
    color: 'green',
  },
}
