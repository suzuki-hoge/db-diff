import type { Meta, StoryObj } from '@storybook/react'

import { IconHelp } from './IconHelp'

const meta = {
  title: 'Atoms/IconHelp',
  component: IconHelp,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconHelp>

export default meta
type Story = StoryObj<typeof meta>

export const Small: Story = {
  args: {
    variant: 'small',
  },
}

export const Medium: Story = {
  args: {
    variant: 'medium',
  },
}

export const Large: Story = {
  args: {
    variant: 'large',
  },
}
