import type { Meta, StoryObj } from '@storybook/react'

import { IconHide } from './IconHide'

const meta = {
  title: 'Atoms/IconHide',
  component: IconHide,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconHide>

export default meta
type Story = StoryObj<typeof meta>

export const Small: Story = {
  args: {
    variant: 'small',
    hide: true,
  },
}

export const Medium: Story = {
  args: {
    variant: 'medium',
    hide: true,
  },
}

export const Large: Story = {
  args: {
    variant: 'large',
    hide: true,
  },
}
