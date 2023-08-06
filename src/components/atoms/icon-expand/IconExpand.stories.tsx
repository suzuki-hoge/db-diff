import type { Meta, StoryObj } from '@storybook/react'

import { IconExpand } from './IconExpand'

const meta = {
  title: 'Atoms/IconExpand',
  component: IconExpand,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconExpand>

export default meta
type Story = StoryObj<typeof meta>

export const Small: Story = {
  args: {
    variant: 'small',
    expanded: true,
  },
}

export const Medium: Story = {
  args: {
    variant: 'medium',
    expanded: true,
  },
}

export const Large: Story = {
  args: {
    variant: 'large',
    expanded: true,
  },
}
