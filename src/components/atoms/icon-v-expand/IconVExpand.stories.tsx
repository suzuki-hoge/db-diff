import type { Meta, StoryObj } from '@storybook/react'

import { IconVExpand } from './IconVExpand'

const meta = {
  title: 'Atoms/IconVExpand',
  component: IconVExpand,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconVExpand>

export default meta
type Story = StoryObj<typeof meta>

export const Small: Story = {
  args: {
    variant: 'small',
    expanded: true,
    onClick: console.log,
  },
}

export const Medium: Story = {
  args: {
    variant: 'medium',
    expanded: true,
    onClick: console.log,
  },
}

export const Large: Story = {
  args: {
    variant: 'large',
    expanded: true,
    onClick: console.log,
  },
}
