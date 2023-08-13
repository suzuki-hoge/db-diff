import type { Meta, StoryObj } from '@storybook/react'

import { ColorTagCard } from './ColorTagCard'

const meta = {
  title: 'Molecules/ColorTagCard',
  component: ColorTagCard,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof ColorTagCard>

export default meta
type Story = StoryObj<typeof meta>

export const Red: Story = {
  args: {
    label: 'My Laravel Project',
    color: 'yellow',
  },
}

export const Green: Story = {
  args: {
    label: 'Tutorial App',
    color: 'green',
  },
}

export const Blue: Story = {
  args: {
    label: '副業のやつ ( RoR )',
    color: 'red',
  },
}
